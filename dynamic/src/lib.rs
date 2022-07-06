#![feature(repr_simd)]
#![allow(non_upper_case_globals)]
pub mod consts;
pub mod ext;
pub mod frame_info;
pub mod game_modes;
mod modules;
pub mod offsets;
pub mod singletons;
pub mod util;

#[macro_use]
extern crate modular_bitfield;

pub use hdr_macros::{export, hash40, import, import_noreturn};

pub use hdr_macros as macros;

pub use frame_info::*;
pub use modules::*;
