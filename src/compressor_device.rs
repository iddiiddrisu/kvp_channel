use biquad::Biquad;
use nih_plug::buffer::Buffer;
use nih_plug::{plugin, prelude::*};
use std::{
    num::NonZero,
    sync::Arc,
};
use crate::compressor::{Compressor, Ratio, ReactionSpeed};
use crate::device::Device;

impl Default for CompressorDevice {
    fn default() -> Self {
        CompressorDevice::new()
    }
}

// impl Plugin for CompressorDevice {
//     const NAME: &'static str = "KVP Eq";

//     const VENDOR: &'static str = "KVP Studios";

//     const URL: &'static str = "www.www.com";

//     const EMAIL: &'static str = "x@x.y";

//     const VERSION: &'static str = "0.03-12-experimental";

//     const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
//         main_input_channels: NonZero::new(2),
//         main_output_channels: NonZero::new(2),
//         ..AudioIOLayout::const_default()
//     }];

//     type SysExMessage = ();

//     type BackgroundTask = ();

//     fn params(&self) -> std::sync::Arc<dyn nih_plug::prelude::Params> {
//         self.compressor_params.clone()
//     }

//     fn process(
//         &mut self,
//         buffer: &mut nih_plug::prelude::Buffer,
//         aux: &mut nih_plug::prelude::AuxiliaryBuffers,
//         context: &mut impl nih_plug::prelude::ProcessContext<Self>,
//     ) -> nih_plug::prelude::ProcessStatus {
//         self.update(context.transport().sample_rate);

//         self.run(buffer);
//         ProcessStatus::Normal
//     }

//     fn reset(&mut self) {
//         self.reset_state();
//     }
// }

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
    // compressor_params: Arc<CompressorDeviceParams>,
}

impl CompressorDevice {
    fn new() -> Self {
        Self {
            compressor: Compressor::new(44000.0),
            // compressor_params: Arc::new(CompressorDeviceParams::new()),
        }
    }
}

impl Device for CompressorDevice {

    type Params = CompressorDeviceParams;
    fn update(&mut self, sample_rate: f32, _compressor_params: &CompressorDeviceParams) {
        self.compressor.sample_rate = sample_rate;
        self.compressor.threshold = _compressor_params.threshold.value();
        self.compressor.ratio = _compressor_params.ratio.value();
        // self.compressor.reaction_speed = self.compressor_params.time.value();
        self.compressor.set_reaction_speed(_compressor_params.time.value())
    }

    fn run(&mut self, input: &mut Buffer) {
        for mut sample_channels in input.iter_samples() {
            for (idx, sample) in sample_channels.iter_mut().enumerate() {
                *sample = self.compressor.process(*sample);
            }
        }
    }

    fn reset_state(&mut self) {
        self.compressor.reset();
    }
}