use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ParamSlider;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::device::KVPChannelPluginParams;

#[derive(Lens)]
struct Data {
    params: Arc<KVPChannelPluginParams>,
}

impl Model for Data {}

// Main editor creation function that's called by the plugin
pub(crate) fn create_editor(
    params: Arc<KVPChannelPluginParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |ctx, _| {
        assets::register_noto_sans_regular(ctx);
        assets::register_noto_sans_bold(ctx);

        Data {
            params: params.clone(),
        }
        .build(ctx);

        VStack::new(ctx, |cx| {
            Label::new(cx, "KVP CHANNEL")
                .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                .font_size(24.0)
                .height(Pixels(40.0))
                .text_align(TextAlign::Center);

            // Main container with three sections
            HStack::new(cx, |cx| {
                // EQ Section
                VStack::new(cx, |cx| {
                    Label::new(cx, "EQ")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(18.0)
                        .height(Pixels(30.0))
                        .text_align(TextAlign::Center);

                    // Input Section
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Input")
                            .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                            .font_size(16.0)
                            .height(Pixels(20.0));

                        // Input Gain
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_gain);
                        })
                        .height(Pixels(30.0));

                        // Low Cut
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Low Cut").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_highpass);
                        })
                        .height(Pixels(30.0));

                        ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_highpass_mode)
                            .height(Pixels(25.0));

                        // High Cut
                        HStack::new(cx, |cx| {
                            Label::new(cx, "High Cut").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_lowpass);
                        })
                        .height(Pixels(30.0));

                        ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_lowpass_mode)
                            .height(Pixels(25.0));
                    })
                    .border_color(Color::aqua())
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(5.0))
                    .child_space(Stretch(1.0));

                    // Pull Section
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Pull")
                            .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                            .font_size(16.0)
                            .height(Pixels(20.0));

                        // Low Shelf
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Low Shelf").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowshelf);
                        })
                        .height(Pixels(30.0));

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowshelf_gain);
                        })
                        .height(Pixels(30.0));

                        // Low Pull
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Low Pull").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowpull);
                        })
                        .height(Pixels(30.0));

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowpull_gain);
                        })
                        .height(Pixels(30.0));

                        // High Pull
                        HStack::new(cx, |cx| {
                            Label::new(cx, "High Pull").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_highpull);
                        })
                        .height(Pixels(30.0));

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_highpull_gain);
                        })
                        .height(Pixels(30.0));
                    })
                    .border_color(Color::aqua())
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(5.0))
                    .child_space(Stretch(1.0));

                    // Push Section
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Push")
                            .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                            .font_size(16.0)
                            .height(Pixels(20.0));

                        // Push Gain
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_gain);
                        })
                        .height(Pixels(30.0));

                        // Overtone Push
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Overtone").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_overtone_push);
                        })
                        .height(Pixels(30.0));

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_overtone_push_gain);
                        })
                        .height(Pixels(30.0));

                        // Tonal Push
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Tonal").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_tonal_push);
                        })
                        .height(Pixels(30.0));

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain").width(Pixels(80.0));
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_tonal_push_gain);
                        })
                        .height(Pixels(30.0));
                    })
                    .border_color(Color::aqua())
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(5.0))
                    .child_space(Stretch(1.0));
                })
                .width(Percentage(33.0))
                .border_color(Color::aqua())
                .border_width(Pixels(1.0))
                .border_radius(Pixels(5.0));

                // Compressor Section
                VStack::new(cx, |cx| {
                    Label::new(cx, "COMPRESSOR")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(18.0)
                        .height(Pixels(30.0))
                        .text_align(TextAlign::Center);

                    // Preset
                    Label::new(cx, "Preset")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(16.0)
                        .height(Pixels(20.0));

                    ParamSlider::new(cx, Data::params, |params| &params.compressor_params.preset)
                        .height(Pixels(30.0));

                    // Threshold
                    Label::new(cx, "Threshold")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(16.0)
                        .height(Pixels(20.0));

                    ParamSlider::new(cx, Data::params, |params| &params.compressor_params.threshold)
                        .height(Pixels(30.0));

                    // Ratio
                    Label::new(cx, "Ratio")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(16.0)
                        .height(Pixels(20.0));

                    ParamSlider::new(cx, Data::params, |params| &params.compressor_params.ratio)
                        .height(Pixels(30.0));
                })
                .width(Percentage(33.0))
                .border_color(Color::aqua())
                .border_width(Pixels(1.0))
                .border_radius(Pixels(5.0));

                // Colorizer Section
                VStack::new(cx, |cx| {
                    Label::new(cx, "COLORIZER")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(18.0)
                        .height(Pixels(30.0))
                        .text_align(TextAlign::Center);

                    // Type
                    Label::new(cx, "Type")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(16.0)
                        .height(Pixels(20.0));

                    ParamSlider::new(cx, Data::params, |params| &params.colorizer_params.color_type)
                        .height(Pixels(30.0));

                    // Intensity
                    Label::new(cx, "Intensity")
                        .font_family(vec![FamilyOwned::Name(String::from("Noto Sans Bold"))])
                        .font_size(16.0)
                        .height(Pixels(20.0));

                    ParamSlider::new(cx, Data::params, |params| &params.colorizer_params.intensity)
                        .height(Pixels(30.0));
                })
                .width(Percentage(33.0))
                .border_color(Color::aqua())
                .border_width(Pixels(1.0))
                .border_radius(Pixels(5.0));
            })
            .height(Percentage(95.0))
            .child_space(Stretch(1.0));
        })
        .background_color(Color::aqua())
        .width(Pixels(800.0))
        .height(Pixels(540.0));
    })
}