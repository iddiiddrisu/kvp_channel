use nih_plug::buffer::Buffer;
use nih_plug::prelude::*;
use std::{
    num::NonZero,
    sync::Arc,
};

use crate::colorizor_device::{ColorizerDevice,ColorizerDeviceParams};
use crate::compressor_device::{CompressorDevice, CompressorDeviceParams};
use crate::eq_device::{EqDevice, EqDeviceParams};


pub trait Device {
    type Params: Params;
    fn run(&mut self, input: &mut Buffer);
    fn update(&mut self, sample_rate: f32, params: &Self::Params);
    fn reset_state(&mut self);
}

// Create a Plugin Implementation of the various devices put together in a chain. 
// FIrst we need a struct to hold this together. 

pub struct KVPChannelPlugin {
    pub eq: EqDevice,
    pub compressor: CompressorDevice,
    pub colorizer: ColorizerDevice,
    pub params: Arc<KVPChannelPluginParams>
}

impl KVPChannelPlugin {
    pub fn update(&mut self, sample_rate: f32) {
        self.eq.update(sample_rate, &self.params.eq_params);
        self.compressor.update(sample_rate, &self.params.compressor_params);
        self.colorizer.update(sample_rate, &self.params.colorizer_params);
    }
}

#[derive(Params)]
pub struct KVPChannelPluginParams {
    #[nested]
    pub eq_params: Arc<EqDeviceParams>,
    #[nested]
    pub compressor_params: Arc<CompressorDeviceParams>,
    #[nested]
    pub colorizer_params: Arc<ColorizerDeviceParams>,
}

impl Default for KVPChannelPluginParams {
    fn default() -> Self {
        Self {
            eq_params: Arc::new(EqDeviceParams::new()),
            compressor_params: Arc::new(CompressorDeviceParams::new()),
            colorizer_params: Arc::new(ColorizerDeviceParams::new()),
        }
    }
}

impl Default for KVPChannelPlugin {
/// Creates a default instance of `KVPChannelPlugin` with initialized 
/// equalization, compression, and colorization devices using default parameters.

    fn default() -> Self {
        Self { eq: EqDevice::new(44000.0), compressor: CompressorDevice::default(), colorizer: ColorizerDevice::default() , params: Arc::new(KVPChannelPluginParams::default())}
    }
}

impl Plugin for KVPChannelPlugin {
    const NAME: &'static str = "KVP Channel";

    const VENDOR: &'static str = "KVP Studios";

    const URL: &'static str = "Ops";

    const EMAIL: &'static str = "Ops";

    const VERSION: &'static str = "1.0.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZero::new(2),
        main_output_channels: NonZero::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn reset(&mut self) {
        self.eq.reset_state();
        self.compressor.reset_state();
        self.colorizer.reset_state();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        self.update(context.transport().sample_rate);
        self.eq.run(buffer);
        self.compressor.run(buffer);
        self.colorizer.run(buffer);
        ProcessStatus::Normal
    }
}
