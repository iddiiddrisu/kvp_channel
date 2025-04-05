use nih_plug::buffer::Buffer;
use nih_plug::prelude::*;
use crate::colorizer::{Colorizer, ColorType};
use crate::device::Device;


pub struct ColorizerDevice {
    colorizer: Colorizer,
    // colorizer_params: Arc<ColorizerDeviceParams>,
}

#[derive(Params)]
pub struct ColorizerDeviceParams {
    #[id = "color_intensity"]
    intensity: FloatParam,
    #[id = "color_type"]
    color_type: EnumParam<ColorType>,
}

impl ColorizerDeviceParams {
    pub fn new() -> Self {
        Self {
            intensity: FloatParam::new(
                "Colorizer:Intensity",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            
            color_type: EnumParam::new("Colorizer:Type", ColorType::Warm),
        }
    }
}

impl ColorizerDevice {
    pub fn new() -> Self {
        Self {
            colorizer: Colorizer::new(44100.0),
            // colorizer_params: Arc::new(ColorizerDeviceParams::new()),
        }
    }
}

impl Default for ColorizerDevice {
    fn default() -> Self {
        Self::new()
    }
}

// trait Device {
//     fn run(&mut self, input: &mut Buffer);
//     fn update(&mut self, sample_rate: f32);
//     fn reset_state(&mut self);
// }

impl Device for ColorizerDevice {

    type Params = ColorizerDeviceParams;

    fn update(&mut self, sample_rate: f32, params: &ColorizerDeviceParams) {
        self.colorizer.sample_rate = sample_rate;
        self.colorizer.set_intensity(params.intensity.value());
        self.colorizer.set_color_type(params.color_type.value());
    }

    fn run(&mut self, input: &mut Buffer) {
        for mut sample_channels in input.iter_samples() {
            for sample in sample_channels.iter_mut() {
                *sample = self.colorizer.process(*sample);
            }
        }
    }

    fn reset_state(&mut self) {
        self.colorizer.reset();
    }
}

// impl Plugin for ColorizerDevice {
//     const NAME: &'static str = "KVP Colorizer";
//     const VENDOR: &'static str = "KVP Studios";
//     const URL: &'static str = "www.www.com";
//     const EMAIL: &'static str = "x@x.y";
//     const VERSION: &'static str = "1.0.0";

//     const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
//         main_input_channels: NonZero::new(2),
//         main_output_channels: NonZero::new(2),
//         ..AudioIOLayout::const_default()
//     }];

//     type SysExMessage = ();
//     type BackgroundTask = ();

//     fn params(&self) -> Arc<dyn Params> {
//         self.colorizer_params.clone()
//     }

//     fn process(
//         &mut self,
//         buffer: &mut Buffer,
//         _aux: &mut AuxiliaryBuffers,
//         context: &mut impl ProcessContext<Self>,
//     ) -> ProcessStatus {
//         self.update(context.transport().sample_rate);
//         self.run(buffer);
//         ProcessStatus::Normal
//     }

//     fn reset(&mut self) {
//         self.reset_state();
//     }
// }