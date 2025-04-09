use nih_plug::buffer::Buffer;
use nih_plug::prelude::*;
use crate::compressor::{Compressor, Ratio, CompressionPreset};
use crate::device::Device;

impl Default for CompressorDevice {
    fn default() -> Self {
        CompressorDevice::new()
    }
}

#[derive(Params)]
pub struct CompressorDeviceParams {
    #[id = "compressor_threshold"]
    pub threshold: FloatParam,
    #[id = "compressor_ratio"]
    pub ratio: EnumParam<Ratio>,
    #[id = "compressor_preset"]
    pub preset: EnumParam<CompressionPreset>
}

impl CompressorDeviceParams {
    pub fn new() -> Self {
        Self {
            threshold: FloatParam::new(
                "Compressor:Threshold", 
                0.0, 
                FloatRange::Linear { min: -32.0, max: 0.0 }
            ),
            ratio: EnumParam::new("Compressor:Ratio", Ratio::Half),
            preset: EnumParam::new("Compressor:Time", CompressionPreset::Drums),
        }
    }
}

pub struct CompressorDevice {
    compressor: Compressor,
}

impl CompressorDevice {
    fn new() -> Self {
        Self {
            compressor: Compressor::new(44100.0),
        }
    }
}

impl Device for CompressorDevice {

    type Params = CompressorDeviceParams;

    fn update(&mut self, sample_rate: f32, _compressor_params: &CompressorDeviceParams) {

        // self.compressor.sample_rate = sample_rate;
        // self.compressor.threshold = _compressor_params.threshold.value();
        // self.compressor.ratio = _compressor_params.ratio.value();
        // self.compressor.set_reaction_speed(_compressor_params.time.value())

        // updated is called at the start of the process loop so should only update when needed. 
        // compare value with current value and only update if different
        if self.compressor.sample_rate == sample_rate  {
            self.compressor.sample_rate = sample_rate;
            self.reset_state();

        }

        self.compressor.threshold = _compressor_params.threshold.value();
        self.compressor.ratio = _compressor_params.ratio.value();
        self.compressor.set_preset(_compressor_params.preset.value());
        
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