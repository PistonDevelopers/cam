#![crate_name = "cam"]
#![deny(missing_docs)]
#![feature(default_type_params)]
#![feature(old_orphan_check)]

//! A library for 3D camera and navigation.

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

