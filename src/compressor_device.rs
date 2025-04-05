use nih_plug::buffer::Buffer;
use nih_plug::prelude::*;
use crate::compressor::{Compressor, Ratio, ReactionSpeed};
use crate::device::Device;

impl Default for CompressorDevice {
    fn default() -> Self {
        CompressorDevice::new()
    }
}

#[derive(Params)]
pub struct CompressorDeviceParams {
    #[id = "compressor_threshold"]
    threshold: FloatParam,
    #[id = "compressor_ratio"]
    ratio: EnumParam<Ratio>,
    #[id = "compressor_time"]
    time: EnumParam<ReactionSpeed>
}

impl CompressorDeviceParams  {
    pub fn new() -> Self {
        Self {
            threshold: FloatParam::new("Amount", 0.0, FloatRange::Linear { min: -32.0, max: 0.0 }),
            ratio: EnumParam::new("Ratio", Ratio::Half),
            time: EnumParam::new("Time", ReactionSpeed::Mid),
        }
    }
}

pub struct CompressorDevice {
    compressor: Compressor,
}

impl CompressorDevice {
    fn new() -> Self {
        Self {
            compressor: Compressor::new(44000.0),
        }
    }
}

impl Device for CompressorDevice {

    type Params = CompressorDeviceParams;
    fn update(&mut self, sample_rate: f32, _compressor_params: &CompressorDeviceParams) {
        self.compressor.sample_rate = sample_rate;
        self.compressor.threshold = _compressor_params.threshold.value();
        self.compressor.ratio = _compressor_params.ratio.value();
        self.compressor.set_reaction_speed(_compressor_params.time.value())
    }

    fn run(&mut self, input: &mut Buffer) {
        for mut sample_channels in input.iter_samples() {
            for (_, sample) in sample_channels.iter_mut().enumerate() {
                *sample = self.compressor.process(*sample);
            }
        }
    }

    fn reset_state(&mut self) {
        self.compressor.reset();
    }
}