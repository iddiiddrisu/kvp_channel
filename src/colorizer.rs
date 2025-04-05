use nih_plug::prelude::Enum;

pub struct Colorizer {
    pub intensity: f32,      // Controls overall effect intensity (0.0 - 1.0)
    pub color_type: ColorType,
    pub sample_rate: f32,
    envelope: f32,
    prev_gain_db: f32,       // Store previous gain for smoothing
    prev_excess_db: f32,     // Store previous excess for effect detection
    attack_coeff: f32,       // Intentionally exaggerated coefficient
    release_coeff: f32,      // Intentionally exaggerated coefficient
    drive: f32,              // Internal drive parameter (derived from intensity)
    saturation: f32,         // Saturation amount
}

#[derive(Copy, Clone, Enum, PartialEq)]
pub enum ColorType {
    Warm,     // Gentle tape-like saturation
    Bright,   // More aggressive with high-end emphasis
    Vintage,  // More midrange focused coloration
}

impl Colorizer {
    pub fn new(sample_rate: f32) -> Self {
        let color_type = ColorType::Warm;
        let attack_coeff = Self::calculate_coefficient(
            Self::attack_time(color_type),
            sample_rate
        );
        let release_coeff = Self::calculate_coefficient(
            Self::release_time(color_type),
            sample_rate
        );

        Colorizer {
            intensity: 0.5,
            color_type,
            sample_rate,
            envelope: 0.0,
            prev_gain_db: 0.0,
            prev_excess_db: 0.0,
            attack_coeff,
            release_coeff,
            drive: 1.0,
            saturation: 0.2,
        }
    }

    // Intentionally create time constants that will produce coloration
    fn calculate_coefficient(time_in_seconds: f32, sample_rate: f32) -> f32 {
        (-1.0 / (time_in_seconds * sample_rate)).exp()
    }

    pub fn attack_time(color: ColorType) -> f32 {
        match color {
            ColorType::Warm => 0.008,     // 8ms - gentler attack
            ColorType::Bright => 0.002,   // 2ms - fast attack for brightness
            ColorType::Vintage => 0.015,  // 15ms - slower attack for vintage feel
        }
    }
    
    pub fn release_time(color: ColorType) -> f32 {
        match color {
            ColorType::Warm => 0.12,     // 120ms - smooth release
            ColorType::Bright => 0.06,   // 60ms - faster release
            ColorType::Vintage => 0.2,   // 200ms - longer release for more 'glue'
        }
    }

    pub fn set_color_type(&mut self, color: ColorType) {
        if self.color_type != color {
            self.color_type = color;
            self.update_coefficients();
        }
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 1.0);
        // Update derived parameters
        self.drive = 1.0 + (intensity * 3.0); // 1.0 to 4.0
        self.saturation = intensity * 0.4;    // 0.0 to 0.4
    }

    fn update_coefficients(&mut self) {
        self.attack_coeff = Self::calculate_coefficient(
            Self::attack_time(self.color_type),
            self.sample_rate
        );
        self.release_coeff = Self::calculate_coefficient(
            Self::release_time(self.color_type),
            self.sample_rate
        );
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // First apply input drive to increase level and introduce saturation
        let driven = input * self.drive;
        
        // Apply saturation - soft clipping
        let saturated = self.saturate(driven);
        
        // Detect level (in dB) for dynamic processing
        let input_db = linear_to_db(saturated.abs());
        
        // Threshold is relative to color type
        let threshold = match self.color_type {
            ColorType::Warm => -12.0,
            ColorType::Bright => -24.0,
            ColorType::Vintage => -18.0,
        };
        
        // Calculate excess - how much signal is above threshold
        let excess_db = input_db - threshold;
        
        // Intentionally use asymmetric smoothing for coloration
        let smoothed_excess = if excess_db > self.prev_excess_db {
            // Fast attack
            self.prev_excess_db + self.attack_coeff * (excess_db - self.prev_excess_db)
        } else {
            // Slower release
            self.prev_excess_db + self.release_coeff * (excess_db - self.prev_excess_db)
        };
        self.prev_excess_db = smoothed_excess;

        // Apply frequency-specific processing based on color type and level
        let processed = match self.color_type {
            ColorType::Warm => {
                // For warm, boost low-mids when signal is strong
                let boost = if smoothed_excess > 0.0 {
                    db_to_linear(smoothed_excess * 0.3 * self.intensity)
                } else {
                    1.0
                };
                saturated * boost
            },
            ColorType::Bright => {
                // For bright, add harmonic excitement on peaks
                if smoothed_excess > 0.0 {
                    // Add subtle harmonics based on level
                    let harmonic_amount = (smoothed_excess / 24.0) * self.intensity;
                    saturated + (saturated * saturated * harmonic_amount) - (saturated * saturated * saturated * harmonic_amount * 0.3)
                } else {
                    saturated
                }
            },
            ColorType::Vintage => {
                // For vintage, apply mild compression with saturation
                if smoothed_excess > 0.0 {
                    let compression = 1.0 - (smoothed_excess * 0.02 * self.intensity);
                    // Mix original and compressed signal
                    let mix_ratio = 0.7; // 70% compressed, 30% original
                    let compressed = saturated * compression;
                    (compressed * mix_ratio) + (saturated * (1.0 - mix_ratio))
                } else {
                    saturated
                }
            }
        };
        
        // Apply output level adjustment to keep overall volume consistent
        processed * (1.0 / (0.5 + (self.intensity * 0.5)))
    }
    
    // Simple saturation function
    fn saturate(&self, input: f32) -> f32 {
        if self.saturation <= 0.0 {
            return input;
        }
        
        // Apply different saturation curves based on color type
        match self.color_type {
            ColorType::Warm => {
                // Soft tape-like saturation
                input * (1.0 + self.saturation * input.abs()).recip()
            },
            ColorType::Bright => {
                // Brighter saturation with harmonic enhancement
                let base = input.tanh() * (1.0 - self.saturation) + input * self.saturation;
                // Add subtle second harmonic
                base + (input * input * 0.1 * self.saturation).tanh()
            },
            ColorType::Vintage => {
                // More aggressive curve with a bit of asymmetry
                let x = input * (1.0 + self.saturation);
                (x / (1.0 + x.abs() + (x * x * 0.1))) * (1.0 + self.saturation * 0.2)
            }
        }
    }
    
    // Reset internal state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.prev_gain_db = 0.0;
        self.prev_excess_db = 0.0;
    }
}

fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0000001 {
        -120.0
    } else {
        20.0 * linear.log10()
    }
}

fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}