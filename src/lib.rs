#![crate_name = "cam"]
#![deny(missing_doc)]
#![feature(default_type_params)]

//! A library for 3D camera and navigation.

extern crate input;
extern crate vecmath;

pub use camera::{
    Camera,
    CameraPerspective,
    model_view_projection,
};
pub use fps_controller::{
    FPSController,
    FPSControllerSettings,
};

mod camera;
mod fps_controller;

