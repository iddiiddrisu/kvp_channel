use std::process::Output;

use biquad::{self, Biquad, Hertz, ToHertz};
use nih_plug::prelude::Enum;

// pub struct InputEq {
//     pub lowpass: biquad::DirectForm1<f32>,
//     pub highpass: biquad::DirectForm1<f32>,
// }

// impl InputEq {
//     pub fn new(sample_rate: f32) -> Self {
//         let fs = sample_rate.hz();
//         let lowpass_coeffs = biquad::Coefficients::<f32>::from_params(
//             biquad::Type::LowPass,
//             fs,
//             20.khz(),
//             biquad::Q_BUTTERWORTH_F32,
//         )
//         .unwrap();
//         let highpass_coeffs = biquad::Coefficients::<f32>::from_params(
//             biquad::Type::HighPass,
//             fs,
//             20.hz(),
//             biquad::Q_BUTTERWORTH_F32,
//         )
//         .unwrap();

//         let lowpass = biquad::DirectForm1::new(lowpass_coeffs);
//         let highpass = biquad::DirectForm1::new(highpass_coeffs);
//         Self { lowpass, highpass }
//     }

//     pub fn update_lowpass(&mut self, cutoff: f32, sample_rate: f32) {
//         let fs = sample_rate.hz();
//         let coeffs = biquad::Coefficients::<f32>::from_params(
//             biquad::Type::LowPass,
//             fs,
//             cutoff.hz(),
//             biquad::Q_BUTTERWORTH_F32,
//         )
//         .unwrap();
//         self.lowpass.update_coefficients(coeffs);
//     }

//     pub fn update_highpass(&mut self, cutoff: f32, sample_rate: f32) {
//         let fs = sample_rate.hz();
//         let coeffs = biquad::Coefficients::<f32>::from_params(
//             biquad::Type::HighPass,
//             fs,
//             cutoff.hz(),
//             biquad::Q_BUTTERWORTH_F32,
//         )
//         .unwrap();
//         self.highpass.update_coefficients(coeffs);
//     }

//     //Get reference to the sample and change the sample
//     pub fn process(&mut self, sample: &mut f32) {
//         *sample = self.lowpass.run(*sample);
//         *sample = self.highpass.run(*sample);
//     }

//     pub fn reset(&mut self) {
//         self.lowpass.reset_state();
//         self.highpass.reset_state();
//     }
// }


// use biquad::{self, Biquad, Hertz, ToHertz};

pub struct InputEq {
    // Using multiple filters in series to create steeper slopes
    highpass_filters: Vec<biquad::DirectForm1<f32>>,
    lowpass_filters: Vec<biquad::DirectForm1<f32>>,
    // slope: FilterSlope,
}

// Define available filter slopes
#[derive(Clone, Copy, PartialEq, Enum)]
pub enum FilterSlope {
    Slope12DB,  // 12 dB/octave (2nd order)
    Slope24DB,  // 24 dB/octave (4th order)
    Slope36DB,  // 36 dB/octave (6th order)
    Slope48DB,  // 48 dB/octave (8th order)
}

impl FilterSlope {
    fn num_filters(&self) -> usize {
        match self {
            FilterSlope::Slope12DB => 1,
            FilterSlope::Slope24DB => 2,
            FilterSlope::Slope36DB => 3,
            FilterSlope::Slope48DB => 4,
        }
    }

    fn to_q(&self) -> f32 {
        match self {
            FilterSlope::Slope12DB => 1.0,
            FilterSlope::Slope24DB => 1.2,
            FilterSlope::Slope36DB => 1.4,
            FilterSlope::Slope48DB => 1.8,
        }
    }
}

impl InputEq {
    pub fn new(sample_rate: f32) -> Self {
        let fs = sample_rate.hz();
        let num_filters = FilterSlope::Slope12DB.num_filters();
        // let slope = FilterSlope::Slope12DB;
        
        // Initialize cascaded filters for highpass
        let highpass_filters = (0..num_filters)
            .map(|_| {
                let coeffs = biquad::Coefficients::<f32>::from_params(
                    biquad::Type::HighPass,
                    fs,
                    20.hz(),
                    FilterSlope::Slope12DB.to_q()
                ).unwrap();
                biquad::DirectForm1::new(coeffs)
            })
            .collect();

        // Initialize cascaded filters for lowpass
        let lowpass_filters = (0..num_filters)
            .map(|_| {
                let coeffs = biquad::Coefficients::<f32>::from_params(
                    biquad::Type::LowPass,
                    fs,
                    20000.hz(),
                    FilterSlope::Slope12DB.to_q()
                ).unwrap();
                biquad::DirectForm1::new(coeffs)
            })
            .collect();

        Self {
            highpass_filters,
            lowpass_filters,
            // slope,
        }
    }

    pub fn update_highpass(&mut self, cutoff: f32, sample_rate: f32, mode: FilterSlope) {
        let fs = sample_rate.hz();
        for filter in &mut self.highpass_filters {
            let coeffs = biquad::Coefficients::<f32>::from_params(
                biquad::Type::HighPass,
                fs,
                cutoff.hz(),
                mode.to_q()
            ).unwrap();
            filter.update_coefficients(coeffs);
        }
    }

    pub fn update_lowpass(&mut self, cutoff: f32, sample_rate: f32, mode: FilterSlope) {
        let fs = sample_rate.hz();
        for filter in &mut self.lowpass_filters {
            let coeffs = biquad::Coefficients::<f32>::from_params(
                biquad::Type::LowPass,
                fs,
                cutoff.hz(),
                mode.to_q()
            ).unwrap();
            filter.update_coefficients(coeffs);
        }
    }

    pub fn process(&mut self, sample: &mut f32) {
        // Apply cascaded highpass filters
        for filter in &mut self.highpass_filters {
            *sample = filter.run(*sample);
        }
        
        // Apply cascaded lowpass filters
        for filter in &mut self.lowpass_filters {
            *sample = filter.run(*sample);
        }
    }

    pub fn reset(&mut self) {
        for filter in &mut self.highpass_filters {
            filter.reset_state();
        }
        for filter in &mut self.lowpass_filters {
            filter.reset_state();
        }
    }
}

pub struct PullEq {
    lowshelf: biquad::DirectForm1<f32>,
    low_pull: biquad::DirectForm1<f32>,
    high_pull: biquad::DirectForm1<f32>,
}

impl PullEq {
    pub fn new(sample_rate: f32) -> Self {
        let fs = sample_rate.hz();
        let lowpass_coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::LowShelf(0.0),
            fs,
            300.hz(),
            biquad::Q_BUTTERWORTH_F32,
        )
        .unwrap();
        let low_peaking_coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(0.0),
            fs,
            800.hz(),
            1.2,
        )
        .unwrap();
        let high_peaking_coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(0.0),
            fs,
            3.khz(),
            1.4,
        )
        .unwrap();

        let lowshelf = biquad::DirectForm1::new(lowpass_coeffs);
        let low_pull = biquad::DirectForm1::new(low_peaking_coeffs);
        let high_pull = biquad::DirectForm1::new(high_peaking_coeffs);
        Self {
            lowshelf,
            low_pull,
            high_pull,
        }
    }

    /// Updates the coefficients of the low shelf filter with the given cutoff frequency and gain.
    /// This will change the sound of the low shelf filter.
    pub fn update_lowshelf(&mut self, cutoff: f32, gain: f32, sample_rate: f32) {
        let fs = sample_rate.hz();
        let coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::LowShelf(gain),
            fs,
            cutoff.hz(),
            biquad::Q_BUTTERWORTH_F32,
        )
        .unwrap();
        self.lowshelf.update_coefficients(coeffs);
    }

    /// Updates the coefficients of the low pull filter with the specified cutoff frequency and gain.
    /// This modifies the peaking EQ characteristics of the low pull filter, influencing the tonal balance
    /// by boosting or cutting frequencies around the specified cutoff. The update is based on the given
    /// sample rate.

    pub fn update_lowpull(&mut self, cutoff: f32, gain: f32, sample_rate: f32) {
        let fs = sample_rate.hz();
        let coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(gain),
            fs,
            cutoff.hz(),
            1.2,
        )
        .unwrap();
        self.low_pull.update_coefficients(coeffs);
    }

    /// Updates the coefficients of the high pull filter with the specified cutoff frequency and gain.
    /// This modifies the peaking EQ characteristics of the high pull filter, influencing the tonal balance
    /// by boosting or cutting frequencies around the specified cutoff. The update is based on the given
    /// sample rate.
    pub fn update_highpull(&mut self, cutoff: f32, gain: f32, sample_rate: f32) {
        let fs = sample_rate.hz();
        let coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(gain),
            fs,
            cutoff.hz(),
            1.4,
        )
        .unwrap();
        self.high_pull.update_coefficients(coeffs);
    }

    /// Applies the Pull EQ to the input sample. This is a three stage EQ,
    /// with a low shelf, a low peaking EQ, and a high peaking EQ. The low shelf
    /// is used to control the tone of the low frequencies, the low peaking EQ
    /// is used to control the tone of the lower midrange frequencies, and the
    /// high peaking EQ is used to control the tone of the higher midrange
    /// frequencies. The output of each stage is fed into the next stage, so
    /// that the overall response is the combination of all three stages.
    pub fn process(&mut self, sample: &mut f32) {
        *sample = self.lowshelf.run(*sample);
        *sample = self.low_pull.run(*sample);
        *sample = self.high_pull.run(*sample);
    }

    pub fn reset(&mut self) {
        self.lowshelf.reset_state();
        self.low_pull.reset_state();
        self.high_pull.reset_state();
    }
}

pub struct PushEq {
    overtone_push: biquad::DirectForm1<f32>,
    tonal_push: biquad::DirectForm1<f32>,
}

impl PushEq {
    pub fn new(sample_rate: f32) -> Self {
        let fs = sample_rate.hz();
        let overtone_push_coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::HighShelf(0.0),
            fs,
            4.khz(),
            1.2,
        )
        .unwrap();
        let tonal_push_coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(0.0),
            fs,
            600.hz(),
            1.0,
        )
        .unwrap();

        let overtone_push = biquad::DirectForm1::new(overtone_push_coeffs);
        let tonal_push = biquad::DirectForm1::new(tonal_push_coeffs);
        Self {
            overtone_push,
            tonal_push,
        }
    }

    /// Updates the coefficients of the overtone push filter with the specified cutoff frequency and gain.
    /// This modifies the peaking EQ characteristics of the overtone push filter, influencing the tonal balance
    /// by boosting or cutting frequencies around the specified cutoff. The update is based on the given
    /// sample rate.
    pub fn update_overtone_push(&mut self, cutoff: f32, gain: f32, sample_rate: f32) {
        let fs = sample_rate.hz();
        let coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(gain),
            fs,
            cutoff.hz(),
            1.2,
        )
        .unwrap();
        self.overtone_push.update_coefficients(coeffs);
    }

    /// Updates the coefficients of the tonal push filter with the specified cutoff frequency and gain.
    /// This modifies the peaking EQ characteristics of the tonal push filter, influencing the tonal balance
    /// by boosting or cutting frequencies around the specified cutoff. The update is based on the given
    /// sample rate.
    pub fn update_tonal_push(&mut self, cutoff: f32, gain: f32, sample_rate: f32) {
        let fs = sample_rate.hz();
        let coeffs = biquad::Coefficients::<f32>::from_params(
            biquad::Type::PeakingEQ(gain),
            fs,
            cutoff.hz(),
            1.0,
        )
        .unwrap();
        self.tonal_push.update_coefficients(coeffs);
    }

    /// Applies the Push EQ to the input sample. This is a two stage EQ,
    /// with a high shelf and a peaking EQ. The high shelf is used to control
    /// the tone of the higher frequencies, and the peaking EQ is used to
    /// control the tone of the lower midrange frequencies. The output of each
    /// stage is fed into the next stage, so that the overall response is the
    /// combination of all three stages.
    pub fn process(&mut self, sample: &mut f32) {
        *sample = self.overtone_push.run(*sample);
        *sample = self.tonal_push.run(*sample);
    }

    pub fn reset(&mut self) {
        self.overtone_push.reset_state();
        self.tonal_push.reset_state();
    }
}
