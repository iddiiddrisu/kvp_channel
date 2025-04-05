use nih_plug::prelude::Enum;

pub struct Compressor {
    pub threshold: f32,  // dB
    pub ratio: Ratio,
    pub reaction_speed: ReactionSpeed,
    pub sample_rate: f32,
    pub knee_width: f32, // dB, for soft knee
    envelope: f32,
    prev_gain_db: f32,
    prev_excess_db: f32,
    attack_coeff: f32,
    release_coeff: f32,
    gain_attack_coeff: f32,
    gain_release_coeff: f32,
    lookahead_buffer: Vec<f32>, // Optional lookahead
    lookahead_pos: usize,
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
        let lookahead_samples = (0.005 * sample_rate).round() as usize; // 5ms lookahead
        
        // Calculate coefficients
        let attack_coeff = Self::calculate_coefficient(
            Self::attack_time(reaction_speed),
            sample_rate
        );
        let release_coeff = Self::calculate_coefficient(
            Self::release_time(reaction_speed),
            sample_rate
        );
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
            knee_width: 6.0, // Default 6dB knee width
            envelope: 0.0,
            prev_gain_db: 0.0,
            prev_excess_db: 0.0,
            attack_coeff,
            release_coeff,
            gain_attack_coeff,
            gain_release_coeff,
            lookahead_buffer: vec![0.0; lookahead_samples],
            lookahead_pos: 0,
        }
    }

    fn calculate_coefficient(time_in_seconds: f32, sample_rate: f32) -> f32 {
        (-1.0 / (time_in_seconds * sample_rate)).exp() // More accurate coefficient calculation
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

    // Soft knee calculation
    fn calculate_knee(&self, excess_db: f32, knee_width: f32) -> f32 {
        if excess_db <= -knee_width / 2.0 {
            0.0
        } else if excess_db >= knee_width / 2.0 {
            excess_db
        } else {
            // In the knee region
            (excess_db + knee_width / 2.0).powi(2) / (2.0 * knee_width)
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Store input in lookahead buffer
        let delayed_sample;
        if !self.lookahead_buffer.is_empty() {
            delayed_sample = self.lookahead_buffer[self.lookahead_pos];
            self.lookahead_buffer[self.lookahead_pos] = input;
            self.lookahead_pos = (self.lookahead_pos + 1) % self.lookahead_buffer.len();
        } else {
            delayed_sample = input;
        }
    
        // Level detection on current input for faster response
        let squared = input * input;
        self.envelope = if squared > self.envelope {
            self.envelope + self.attack_coeff * (squared - self.envelope)
        } else {
            self.envelope + self.release_coeff * (squared - self.envelope)
        };
        
        let rms_linear = self.envelope.sqrt();
        let rms_db = linear_to_db(rms_linear);
        
        let excess_db = rms_db - self.threshold;
        let knee_excess = self.calculate_knee(excess_db, self.knee_width);
        
        let smoothed_excess = if knee_excess > self.prev_excess_db {
            self.prev_excess_db + self.attack_coeff * (knee_excess - self.prev_excess_db)
        } else {
            self.prev_excess_db + self.release_coeff * (knee_excess - self.prev_excess_db)
        };
        self.prev_excess_db = smoothed_excess;
    
        // Modified gain calculation to ensure sufficient reduction
        let target_gain_db = if smoothed_excess > 0.0 {
            let ratio = Self::ratio_to_value(self.ratio);
            -smoothed_excess * (1.0 - 1.0/ratio) // Remove tanh to allow deeper compression
        } else {
            0.0
        };
    
        let gain_db = if target_gain_db < self.prev_gain_db {
            self.prev_gain_db + (target_gain_db - self.prev_gain_db) * (1.0 - self.gain_attack_coeff)
        } else {
            self.prev_gain_db + (target_gain_db - self.prev_gain_db) * (1.0 - self.gain_release_coeff)
        };
        self.prev_gain_db = gain_db;
    
        // Apply the gain to the delayed sample
        let gain = db_to_linear(gain_db);
        delayed_sample * gain
    }
    
    // Reset the compressor state
    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.prev_gain_db = 0.0;
        self.prev_excess_db = 0.0;
    }
}

fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0000001 {  // -120dB floor
        -120.0
    } else {
        20.0 * linear.log10()
    }
}

fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}