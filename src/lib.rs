#![crate_name = "cam"]
#![deny(missing_docs)]
#![feature(core, hash, std_misc)]

//! A library for 3D camera and navigation.

#[macro_use]
extern crate bitflags;
extern crate event;
extern crate input;
extern crate vecmath;
extern crate cgmath;

pub use camera::{
    Camera,
    CameraPerspective,
    model_view_projection,
};
pub use first_person::{
    FirstPerson,
    FirstPersonSettings,
};

pub use orbit_zoom_camera::{
    OrbitZoomCamera,
    OrbitZoomCameraSettings,
};

mod camera;
mod first_person;
mod orbit_zoom_camera;

