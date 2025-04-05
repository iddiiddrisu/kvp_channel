use nih_plug::prelude::Enum;

pub struct Compressor {
    pub threshold: f32,  // dB directly
    pub ratio: Ratio,
    pub reaction_speed: ReactionSpeed,
    pub sample_rate: f32,
    pub envelope: f32,
    prev_gain_db: f32,   // Store previous gain for smoothing
    prev_excess_db: f32, // Store previous excess for better detection
    attack_coeff: f32,   // Precomputed coefficient for attack
    release_coeff: f32,  // Precomputed coefficient for release
    gain_attack_coeff: f32,  // Separate coefficient for gain attack
    gain_release_coeff: f32,  // Separate coefficient for gain release
}

#[derive(Copy, Clone, Enum, PartialEq)]
pub enum ReactionSpeed {
    Fast,
    Mid,
    Slow
}

#[derive(Copy, Clone, Enum, PartialEq)]
pub enum Ratio {
    Half,
    Third,
    Quarter,
}

impl Compressor {
    pub fn new(sample_rate: f32) -> Self {
        let reaction_speed = ReactionSpeed::Mid;
        
        // Calculate detection coefficients
        let attack_coeff = Self::calculate_coefficient(
            Self::attack_time(reaction_speed),
            sample_rate
        );
        let release_coeff = Self::calculate_coefficient(
            Self::release_time(reaction_speed),
            sample_rate
        );
        
        // Calculate gain smoothing coefficients (slightly faster than detection)
        let gain_attack_coeff = Self::calculate_coefficient(
            Self::gain_attack_time(reaction_speed),
            sample_rate
        );
        let gain_release_coeff = Self::calculate_coefficient(
            Self::gain_release_time(reaction_speed),
            sample_rate
        );

        Compressor {
            threshold: 0.0,
            ratio: Ratio::Half,
            reaction_speed,
            sample_rate,
            envelope: 0.0,
            prev_gain_db: 0.0,
            prev_excess_db: 0.0,
            attack_coeff,
            release_coeff,
            gain_attack_coeff,
            gain_release_coeff,
        }
    }

    // Convert time in seconds to smoothing coefficient
    fn calculate_coefficient(time_in_seconds: f32, sample_rate: f32) -> f32 {
        (-2.2 / (time_in_seconds * sample_rate)).exp()
    }

    pub fn attack_time(speed: ReactionSpeed) -> f32 {
        match speed {
            ReactionSpeed::Fast => 0.01,  // 10ms
            ReactionSpeed::Mid => 0.025,  // 25ms
            ReactionSpeed::Slow => 0.05   // 50ms
        }
    }
    
    pub fn release_time(speed: ReactionSpeed) -> f32 {
        match speed {
            ReactionSpeed::Fast => 0.05,  // 50ms
            ReactionSpeed::Mid => 0.1,    // 100ms
            ReactionSpeed::Slow => 0.2    // 200ms
        }
    }
    
    // Slightly faster gain attack for more transparent compression
    pub fn gain_attack_time(speed: ReactionSpeed) -> f32 {
        match speed {
            ReactionSpeed::Fast => 0.005,  // 5ms
            ReactionSpeed::Mid => 0.015,   // 15ms
            ReactionSpeed::Slow => 0.03    // 30ms
        }
    }
    
    // Slightly slower gain release to prevent pumping
    pub fn gain_release_time(speed: ReactionSpeed) -> f32 {
        match speed {
            ReactionSpeed::Fast => 0.08,   // 80ms
            ReactionSpeed::Mid => 0.15,    // 150ms
            ReactionSpeed::Slow => 0.25    // 250ms
        }
    }

    fn ratio_to_value(ratio: Ratio) -> f32 {
        match ratio {
            Ratio::Half => 2.0,    // 2:1
            Ratio::Third => 3.0,   // 3:1
            Ratio::Quarter => 4.0, // 4:1
        }
    }

    pub fn set_reaction_speed(&mut self, speed: ReactionSpeed) {
        // update only if there is a change
        if self.reaction_speed != speed {
            self.reaction_speed = speed;
            self.update_coefficients();
        }
    }

    fn update_coefficients(&mut self) {
        self.attack_coeff = Self::calculate_coefficient(
            Self::attack_time(self.reaction_speed),
            self.sample_rate
        );
        self.release_coeff = Self::calculate_coefficient(
            Self::release_time(self.reaction_speed),
            self.sample_rate
        );
        self.gain_attack_coeff = Self::calculate_coefficient(
            Self::gain_attack_time(self.reaction_speed),
            self.sample_rate
        );
        self.gain_release_coeff = Self::calculate_coefficient(
            Self::gain_release_time(self.reaction_speed),
            self.sample_rate
        );
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Convert input to dB, with proper handling of very small values
        let input_db = linear_to_db(input.abs());
        
        // Calculate excess (how much the signal exceeds the threshold)
        let excess_db = input_db - self.threshold;
        
        // Smooth the excess detection using peak detection with different attack/release times
        let smoothed_excess = if excess_db > self.prev_excess_db {
            self.prev_excess_db + self.attack_coeff * (excess_db - self.prev_excess_db)
        } else {
            self.prev_excess_db + self.release_coeff * (excess_db - self.prev_excess_db)
        };
        self.prev_excess_db = smoothed_excess;

        // Only apply compression if we're above threshold
        let target_gain_db = if smoothed_excess > 0.0 {
            // Calculate gain reduction
            let ratio = Self::ratio_to_value(self.ratio);
            -(smoothed_excess * (1.0 - 1.0/ratio))
        } else {
            0.0
        };

        // Smooth the gain changes with different coefficients for attack and release
        // This provides more natural compression characteristics
        let gain_db = if target_gain_db < self.prev_gain_db {
            // We're increasing compression (more negative gain), use attack time
            self.prev_gain_db + self.gain_attack_coeff * (target_gain_db - self.prev_gain_db)
        } else {
            // We're decreasing compression (less negative gain), use release time
            self.prev_gain_db + self.gain_release_coeff * (target_gain_db - self.prev_gain_db)
        };
        self.prev_gain_db = gain_db;

        // Convert gain back to linear and apply
        let gain = db_to_linear(gain_db);
        input * gain
    }
    
    // Reset the compressor state
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