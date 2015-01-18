#![crate_name = "cam"]
#![deny(missing_docs)]
#![allow(unstable)]

//! A library for 3D camera and navigation.

#[macro_use] extern crate rustc_bitflags;
extern crate event;
extern crate input;
extern crate vecmath;

pub use camera::{
    Camera,
    CameraPerspective,
    model_view_projection,
};
pub use first_person::{
    FirstPerson,
    FirstPersonSettings,
};

mod camera;
mod first_person;

