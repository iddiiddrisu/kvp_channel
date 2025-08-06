use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ParamSlider;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::device::KVPChannelPluginParams;

// Define colors for our retrofuture rusted metal theme
const RUST_ORANGE: Color = Color::rgba(194, 107, 36, 1);
const RUST_ORANGE_DARK: Color = Color::rgba(150, 75, 20, 1);
const RUST_ORANGE_LIGHT: Color = Color::rgba(226, 141, 63, 1);
const PANEL_BG: Color = Color::rgba(40, 32, 28, 1);
const TEXT_COLOR: Color = Color::rgba(255, 235, 210, 1);
const BORDER_COLOR: Color = Color::rgba(90, 55, 30, 1);

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
            // Header with title
            Label::new(cx, "KVP CHANNEL")
                .font_size(28.0)
                .height(Pixels(50.0))
                .color(RUST_ORANGE_LIGHT)
                .text_align(TextAlign::Center);

            // Main container with three columns
            HStack::new(cx, |cx| {
                // Left Column - EQ
                VStack::new(cx, |cx| {
                    // EQ Header
                    Label::new(cx, "EQ")
                        .font_size(20.0)
                        .height(Pixels(32.0))
                        .color(TEXT_COLOR)
                        .text_align(TextAlign::Center);

                    // Input Gain
                    VStack::new(cx, |cx| {
                        Label::new(cx, "— INPUT GAIN —")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR);

                        HStack::new(cx, |cx| {
                            Label::new(cx, "Gain")
                                .width(Pixels(80.0))
                                .color(TEXT_COLOR);
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_gain);
                        })
                        .height(Pixels(32.0))
                        .left(Pixels(8.0));
                    })
                    .background_color(PANEL_BG)
                    .border_color(BORDER_COLOR)
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(6.0))
                    .child_space(Stretch(1.0))
                    .top(Pixels(5.0))
                    .bottom(Pixels(5.0))
                    .left(Pixels(5.0))
                    .right(Pixels(5.0));

                    // Input Filters
                    VStack::new(cx, |cx| {
                        Label::new(cx, "— INPUT FILTERS —")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR);

                        // Low Cut
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Low Cut")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_highpass)
                                .height(Pixels(32.0))
                                .left(Pixels(8.0))
                                .right(Pixels(8.0));

                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_highpass_mode)
                                .height(Pixels(25.0))
                                .left(Pixels(8.0))
                                .right(Pixels(8.0));
                        })
                        .top(Pixels(5.0));

                        // High Cut
                        VStack::new(cx, |cx| {
                            Label::new(cx, "High Cut")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_lowpass)
                                .height(Pixels(32.0))
                                .left(Pixels(8.0))
                                .right(Pixels(8.0));

                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.input_eq_lowpass_mode)
                                .height(Pixels(25.0))
                                .left(Pixels(8.0))
                                .right(Pixels(8.0));
                        })
                        .top(Pixels(10.0))
                        .bottom(Pixels(5.0));
                    })
                    .background_color(PANEL_BG)
                    .border_color(BORDER_COLOR)
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(6.0))
                    .child_space(Stretch(1.0))
                    .top(Pixels(5.0))
                    .bottom(Pixels(5.0))
                    .left(Pixels(5.0))
                    .right(Pixels(5.0));

                    // Pull Section
                    VStack::new(cx, |cx| {
                        Label::new(cx, "— PULL EQ —")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR);

                        // Low Shelf
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Low Shelf")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Freq")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowshelf);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));

                            HStack::new(cx, |cx| {
                                Label::new(cx, "Gain")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowshelf_gain);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));
                        })
                        .bottom(Pixels(8.0));

                        // Low Pull
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Low Pull")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Freq")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowpull);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));

                            HStack::new(cx, |cx| {
                                Label::new(cx, "Gain")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_lowpull_gain);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));
                        })
                        .bottom(Pixels(8.0));

                        // High Pull
                        VStack::new(cx, |cx| {
                            Label::new(cx, "High Pull")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Freq")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_highpull);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));

                            HStack::new(cx, |cx| {
                                Label::new(cx, "Gain")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.pull_highpull_gain);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));
                        });
                    })
                    .background_color(PANEL_BG)
                    .border_color(BORDER_COLOR)
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(6.0))
                    .child_space(Stretch(1.0))
                    .top(Pixels(5.0))
                    .bottom(Pixels(5.0))
                    .left(Pixels(5.0))
                    .right(Pixels(5.0));
                })
                .width(Percentage(34.0))
                .left(Pixels(5.0))
                .right(Pixels(5.0));

                // Middle Column - Push EQ and Compressor
                VStack::new(cx, |cx| {
                    // Push Section
                    VStack::new(cx, |cx| {
                        Label::new(cx, "— PUSH EQ —")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR);

                        // Push Gain
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Output Gain")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_gain)
                                .height(Pixels(32.0))
                                .left(Pixels(8.0))
                                .right(Pixels(8.0));
                        })
                        .bottom(Pixels(8.0));

                        // Overtone Push
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Overtone")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Freq")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_overtone_push);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));

                            HStack::new(cx, |cx| {
                                Label::new(cx, "Gain")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_overtone_push_gain);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));
                        })
                        .bottom(Pixels(8.0));

                        // Tonal Push
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Tonal")
                                .color(TEXT_COLOR)
                                .height(Pixels(20.0))
                                .text_align(TextAlign::Center);
                                
                            HStack::new(cx, |cx| {
                                Label::new(cx, "Freq")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_tonal_push);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));

                            HStack::new(cx, |cx| {
                                Label::new(cx, "Gain")
                                    .width(Pixels(40.0))
                                    .color(TEXT_COLOR);
                                ParamSlider::new(cx, Data::params, |params| &params.eq_params.push_tonal_push_gain);
                            })
                            .height(Pixels(32.0))
                            .left(Pixels(8.0));
                        });
                    })
                    .background_color(PANEL_BG)
                    .border_color(BORDER_COLOR)
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(6.0))
                    .child_space(Stretch(1.0))
                    .top(Pixels(37.0)) // Align with first section header in left column
                    .bottom(Pixels(5.0))
                    .left(Pixels(5.0))
                    .right(Pixels(5.0));

                    // Compressor Section
                    VStack::new(cx, |cx| {
                        // Section Header
                        Label::new(cx, "COMPRESSOR")
                            .font_size(20.0)
                            .height(Pixels(32.0))
                            .color(TEXT_COLOR)
                            .text_align(TextAlign::Center);

                        // Preset
                        Label::new(cx, "Preset")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR);

                        ParamSlider::new(cx, Data::params, |params| &params.compressor_params.preset)
                            .height(Pixels(32.0))
                            .left(Pixels(8.0))
                            .right(Pixels(8.0));

                        // Threshold
                        Label::new(cx, "Threshold")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR)
                            .top(Pixels(15.0));

                        ParamSlider::new(cx, Data::params, |params| &params.compressor_params.threshold)
                            .height(Pixels(32.0))
                            .left(Pixels(8.0))
                            .right(Pixels(8.0));

                        // Ratio
                        Label::new(cx, "Ratio")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR)
                            .top(Pixels(15.0));

                        ParamSlider::new(cx, Data::params, |params| &params.compressor_params.ratio)
                            .height(Pixels(32.0))
                            .left(Pixels(8.0))
                            .right(Pixels(8.0));
                    })
                    .background_color(PANEL_BG)
                    .border_color(BORDER_COLOR)
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(6.0))
                    .child_space(Stretch(1.0))
                    .top(Pixels(5.0))
                    .bottom(Pixels(5.0))
                    .left(Pixels(5.0))
                    .right(Pixels(5.0));
                })
                .width(Percentage(33.0))
                .left(Pixels(5.0))
                .right(Pixels(5.0));

                // Right Column - Colorizer
                VStack::new(cx, |cx| {
                    // Section Header
                    Label::new(cx, "COLORIZER")
                        .font_size(20.0)
                        .height(Pixels(32.0))
                        .color(TEXT_COLOR)
                        .text_align(TextAlign::Center);

                    VStack::new(cx, |cx| {
                        // Type
                        Label::new(cx, "Type")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR);

                        ParamSlider::new(cx, Data::params, |params| &params.colorizer_params.color_type)
                            .height(Pixels(32.0))
                            .left(Pixels(8.0))
                            .right(Pixels(8.0));

                        // Intensity
                        Label::new(cx, "Intensity")
                            .font_size(16.0)
                            .height(Pixels(24.0))
                            .color(TEXT_COLOR)
                            .top(Pixels(15.0));

                        ParamSlider::new(cx, Data::params, |params| &params.colorizer_params.intensity)
                            .height(Pixels(32.0))
                            .left(Pixels(8.0))
                            .right(Pixels(8.0));
                            
                        // Decorative knob visualization
                        Label::new(cx, "COLOR TONE")
                            .font_size(14.0)
                            .height(Pixels(20.0))
                            .color(TEXT_COLOR)
                            .text_align(TextAlign::Center)
                            .top(Pixels(30.0));
                            
                        Element::new(cx)
                            .background_color(RUST_ORANGE)
                            .width(Pixels(100.0))
                            .height(Pixels(100.0))
                            .border_radius(Percentage(50.0))
                            .border_color(BORDER_COLOR)
                            .border_width(Pixels(2.0))
                            .top(Pixels(10.0));
                            
                        // Visual EQ display representation
                        Label::new(cx, "FREQUENCY RESPONSE")
                            .font_size(14.0)
                            .height(Pixels(20.0))
                            .color(TEXT_COLOR)
                            .text_align(TextAlign::Center)
                            .top(Pixels(30.0));
                            
                        Element::new(cx)
                            .background_color(PANEL_BG)
                            .width(Percentage(90.0))
                            .height(Pixels(120.0))
                            .border_radius(Pixels(4.0))
                            .border_color(BORDER_COLOR)
                            .border_width(Pixels(1.0))
                            .top(Pixels(5.0));
                    })
                    .background_color(PANEL_BG)
                    .border_color(BORDER_COLOR)
                    .border_width(Pixels(1.0))
                    .border_radius(Pixels(6.0))
                    .child_space(Stretch(1.0))
                    .top(Pixels(37.0)) // Align with first section in left column
                    .bottom(Pixels(5.0))
                    .left(Pixels(5.0))
                    .right(Pixels(5.0))
                    .height(Percentage(95.0));
                })
                .width(Percentage(33.0))
                .left(Pixels(5.0))
                .right(Pixels(5.0));
            })
            .height(Percentage(90.0))
            .child_space(Stretch(1.0))
            .top(Pixels(10.0))
            .bottom(Pixels(10.0))
            .left(Pixels(10.0))
            .right(Pixels(10.0));
            
            // Footer
            Label::new(cx, "KVP STUDIOS © 2025")
                .font_size(12.0)
                .color(RUST_ORANGE_LIGHT)
                .text_align(TextAlign::Center)
                .height(Pixels(20.0));
        })
        .background_color(RUST_ORANGE_DARK);
    })
}