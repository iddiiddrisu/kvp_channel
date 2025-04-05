use crate::device;
use crate::eq::FilterSlope;
use crate::{
    eq::{InputEq, PullEq, PushEq},
    gain,
};
use device::Device;

use nih_plug::buffer::Buffer;
use nih_plug::prelude::*;

pub struct EqDevice {
    input_gain: gain::GainStage,
    input_eq: Vec<InputEq>,
    pull_eq: Vec<PullEq>,
    push_gain: gain::GainStage,
    push_eq: Vec<PushEq>,
}

const NUM_CHANNELS: i8 = 2;

impl EqDevice {
    /// Creates a new instance of `EqDevice` with initialized gain stages, equalization stages,
    /// and parameter settings based on the specified sample rate.

    pub fn new(sample_rate: f32) -> Self {
        EqDevice {
            input_gain: gain::GainStage::new(),
            input_eq: (0..NUM_CHANNELS)
                .map(|_| InputEq::new(sample_rate))
                .collect(),
            pull_eq: (0..NUM_CHANNELS)
                .map(|_| PullEq::new(sample_rate))
                .collect(),
            push_gain: gain::GainStage::new(),
            push_eq: (0..NUM_CHANNELS)
                .map(|_| PushEq::new(sample_rate))
                .collect(),
        }
    }
}

impl Device for EqDevice {
    /// Applies the full equalization process to the provided input buffer, using the most
    /// recently updated parameter values. Returns the processed buffer.
    /// 
    
    type Params = EqDeviceParams;

    fn run(&mut self, input: &mut Buffer) {
        for mut sample_channels in input.iter_samples() {
            for (idx, sample) in sample_channels.iter_mut().enumerate() {
                self.input_gain.process(sample);
                self.input_eq[idx].process(sample);
                self.pull_eq[idx].process(sample);
                self.push_gain.process(sample);
                self.push_eq[idx].process(sample);
            }
        }
    }

    /// Updates the internal state of the equalization device based on the provided sample rate and
    /// the current parameter values. This should be called whenever the sample rate changes or
    /// the user changes a parameter value.
    fn update(&mut self, sample_rate: f32, _eq_params: &EqDeviceParams) {
        self.input_gain.update(_eq_params.input_gain.value());
        self.input_eq.iter_mut().for_each(|input| {
            input.update_highpass(_eq_params.input_eq_highpass.value(), sample_rate, _eq_params.input_eq_highpass_mode.value());
            input.update_lowpass(_eq_params.input_eq_lowpass.value(), sample_rate, _eq_params.input_eq_lowpass_mode.value());
        });
        self.pull_eq.iter_mut().for_each(|pull| {
            pull.update_lowshelf(
                _eq_params.pull_lowshelf.value(),
                _eq_params.pull_lowshelf_gain.value(),
                sample_rate,
            );
            pull.update_lowpull(
                _eq_params.pull_lowpull.value(),
                _eq_params.pull_lowpull_gain.value(),
                sample_rate,
            );
            pull.update_highpull(
                _eq_params.pull_highpull.value(),
                _eq_params.pull_highpull_gain.value(),
                sample_rate,
            );
        });
        self.push_gain.update(_eq_params.push_gain.value());
        self.push_eq.iter_mut().for_each(|push| {
            push.update_overtone_push(
                _eq_params.push_overtone_push.value(),
                _eq_params.push_overtone_push_gain.value(),
                sample_rate,
            );
            push.update_tonal_push(
                _eq_params.push_tonal_push.value(),
                _eq_params.push_tonal_push_gain.value(),
                sample_rate,
            );
        });
    }

    fn reset_state(&mut self) {
        self.input_eq.iter_mut().for_each(|input| input.reset());
        self.pull_eq.iter_mut().for_each(|pull| pull.reset());
        self.push_eq.iter_mut().for_each(|push| push.reset());
    }
}

#[derive(Params)]
pub struct EqDeviceParams {
    #[id = "input_gain"]
    input_gain: FloatParam,
    #[id = "input_eq_lowpass_mode"]
    input_eq_lowpass_mode: EnumParam<FilterSlope>,
    #[id = "input_eq_lowpass"]
    input_eq_lowpass: FloatParam,
    #[id = "input_eq_highpass_mode"]
    input_eq_highpass_mode: EnumParam<FilterSlope>,
    #[id = "input_eq_highpass"]
    input_eq_highpass: FloatParam,
    #[id = "pull_lowshelf"]
    pull_lowshelf: FloatParam,
    #[id = "pull_lowshelf_gain"]
    pull_lowshelf_gain: FloatParam,
    #[id = "pull_lowpull"]
    pull_lowpull: FloatParam,
    #[id = "pull_lowpull_gain"]
    pull_lowpull_gain: FloatParam,
    #[id = "pull_highpull"]
    pull_highpull: FloatParam,
    #[id = "pull_highpull_gain"]
    pull_highpull_gain: FloatParam,
    #[id = "push_gain"]
    push_gain: FloatParam,
    #[id = "push_overtone_push"]
    push_overtone_push: FloatParam,
    #[id = "push_overtone_push_gain"]
    push_overtone_push_gain: FloatParam,
    #[id = "push_tonal_push"]
    push_tonal_push: FloatParam,
    #[id = "push_tonal_push_gain"]
    push_tonal_push_gain: FloatParam,
}

impl EqDeviceParams {
    pub fn new() -> Self {
        Self {
            input_gain: FloatParam::new(
                "Input:Trim:Gain",
                1.0,
                FloatRange::Skewed {
                    min: 0.02,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(0.02, 7.0),
                },
            ),
            input_eq_lowpass: FloatParam::new(
                "Input:Trim:HighCut:Freq",
                20000.0,
                FloatRange::Linear {
                    min: 8000.0,
                    max: 20000.0,
                },
            ),
            input_eq_highpass: FloatParam::new(
                "Input:Trim:LowCut:Freq",
                20.0,
                FloatRange::Linear {
                    min: 20.0,
                    max: 200.0,
                },
            ),
            pull_lowshelf: FloatParam::new(
                "EQ:Pull:LowShelf:Freq",
                300.0,
                FloatRange::Linear {
                    min: 100.0,
                    max: 500.0,
                },
            ),
            pull_lowshelf_gain: FloatParam::new(
                "EQ:Pull:LowShelf:Gain",
                1.0,
                FloatRange::Skewed {
                    min: -7.0,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(-7.0, 7.0),
                },
            ),
            pull_lowpull: FloatParam::new(
                "EQ:Pull:LowPull:Freq",
                800.0,
                FloatRange::Linear {
                    min: 200.0,
                    max: 1000.0,
                },
            ),
            pull_lowpull_gain: FloatParam::new(
                "EQ:Pull:LowPull:Gain",
                0.0,
                FloatRange::Skewed {
                    min: -7.0,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(-7.0, 7.0),
                },
            ),
            pull_highpull: FloatParam::new(
                "EQ:Pull:HighPull:Freq",
                3000.0,
                FloatRange::Linear {
                    min: 1000.0,
                    max: 5000.0,
                },
            ),
            pull_highpull_gain: FloatParam::new(
                "EQ:Pull:HighPull:Gain",
                0.0,
                FloatRange::Skewed {
                    min: -7.0,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(-7.0, 7.0),
                },
            ),
            push_gain: FloatParam::new(
                "EQ:Push:Trim:Gain",
                1.0,
                FloatRange::Skewed {
                    min: 0.02,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(0.02, 7.0),
                },
            ),
            push_overtone_push: FloatParam::new(
                "EQ:Push:Overtone:Freq",
                4000.0,
                FloatRange::Linear {
                    min: 3000.0,
                    max: 12000.0,
                },
            ),
            push_tonal_push: FloatParam::new(
                "EQ:Push:Tonal:Freq",
                500.0,
                FloatRange::Linear {
                    min: 470.0,
                    max: 1200.0,
                },
            ),
            push_overtone_push_gain: FloatParam::new(
                "EQ:Push:Overtone:Gain",
                1.0,
                FloatRange::Skewed {
                    min: -7.0,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(-7.0, 7.0),
                },
            ),
            push_tonal_push_gain: FloatParam::new(
                "EQ:Push:Tonal:Gain",
                1.0,
                FloatRange::Skewed {
                    min: -7.0,
                    max: 7.0,
                    factor: FloatRange::gain_skew_factor(-7.0, 7.0),
                },
            ),
            input_eq_lowpass_mode: EnumParam::new("Input:Trim:HighCut:Mode", FilterSlope::Slope48DB),
            input_eq_highpass_mode: EnumParam::new("Input:Trim:LowCut:Mode", FilterSlope::Slope48DB),
        }
    }
}

impl Default for EqDevice {
    fn default() -> Self {
        EqDevice::new(44100.0)
    }
}
