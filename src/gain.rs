// gain staging area

use nih_plug::params::FloatParam;

pub struct GainStage {
    pub gain: f32,
    // pub gain_param: nih_plug::params::FloatParam
}

impl GainStage {
    pub fn new() -> Self {
        GainStage { gain: 1.0 }
    }

    pub fn process(&mut self, sample: &mut f32) {
        *sample = *sample * self.gain
    }

    pub fn update(&mut self, gain: f32) {
        self.gain = gain
    }
}
