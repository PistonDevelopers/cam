#![crate_name = "cam"]
#![deny(missing_docs)]

//! A library for 3D camera and navigation.

extern crate vecmath;
extern crate quaternion;
extern crate num;

pub use camera::{
    Camera,
    CameraPerspective,
    model_view_projection,
};

mod camera;
