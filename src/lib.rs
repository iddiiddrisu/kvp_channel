use device::KVPChannelPlugin;
use nih_plug::{
    nih_export_clap,
    prelude::{ClapFeature, ClapPlugin},
};

mod device;
mod eq;
mod gain;
mod compressor;
mod colorizer;
mod colorizor_device;
mod eq_device;
mod compressor_device;
mod ui;

impl ClapPlugin for KVPChannelPlugin {
    const CLAP_ID: &'static str = "com.kvp.studio";

    const CLAP_DESCRIPTION: Option<&'static str> = Some("The Eq for your vocals");

    const CLAP_MANUAL_URL: Option<&'static str> = Some("nonmaguy");

    const CLAP_SUPPORT_URL: Option<&'static str> = Some("Ha, Thats a good one");

    const CLAP_FEATURES: &'static [nih_plug::prelude::ClapFeature] =
        &[ClapFeature::AudioEffect, ClapFeature::Equalizer];
}

nih_export_clap!(KVPChannelPlugin);


// build command
// cargo xtask bundle KVPChannelPlugin --release