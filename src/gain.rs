
pub struct GainStage {
    pub gain: f32,
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
