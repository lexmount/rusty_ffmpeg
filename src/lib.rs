mod avutil;

#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes,
    unnecessary_transmutes,
    clippy::all
)]
#[cfg_attr(
    feature = "bundled_h264",
    allow(unknown_lints, suspicious_runtime_symbol_definitions)
)]
pub mod ffi {
    #[cfg(feature = "ffmpeg6")]
    pub use crate::avutil::channel_layout::*;
    pub use crate::avutil::{_avutil::*, common::*, error::*, pixfmt::*, rational::*};
    include!(concat!(env!("OUT_DIR"), "/binding.rs"));
}
