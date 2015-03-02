#![crate_name = "cam"]
#![deny(missing_docs)]
#![feature(core, std_misc)]

//! A library for 3D camera and navigation.

#[macro_use]
extern crate vecmath;
extern crate quaternion;

pub use camera::{
    Camera,
    CameraPerspective,
    model_view_projection,
};

mod camera;
