//! 
//! A 3dsMax / Blender style camera that orbits about a target position
//!

use event::GenericEvent;
use std::ops::Mul;
use cgmath::*;
use { input, Camera };

use input::Button::{Keyboard, Mouse};
use input::keyboard::Key;
use input::mouse::MouseButton;


bitflags!(flags Keys: u8 {
    const ZOOM  = 0b00000001,
    const PAN   = 0b00000010,
    const ORBIT = 0b00000100,
});

///
/// Specifies key bindings and speed modifiers for OrbitZoomCamera
///
pub struct OrbitZoomCameraSettings<T=f32> {

    /// Which button to press to orbit with mouse
    pub orbit_button: input::Button,

    /// Which button to press to zoom with mouse
    pub zoom_button: input::Button,

    /// Which button to press to pan with mouse
    pub pan_button: input::Button,

    /// Modifier for orbiting speed (arbitrary unit)
    pub orbit_speed: T,

    /// Modifier for panning speed (arbitrary unit)
    pub pan_speed: T,

    /// Modifier for zoom speed (arbitrary unit)
    pub zoom_speed: T,
}

impl OrbitZoomCameraSettings {

    ///
    /// Clicking and dragging OR two-finger scrolling will orbit camera,
    /// with LShift as pan modifer and LCtrl as zoom modifier
    ///
    pub fn default() -> OrbitZoomCameraSettings {
        OrbitZoomCameraSettings {
            orbit_button : Mouse(MouseButton::Left),
            zoom_button : Keyboard(Key::LCtrl),
            pan_button : Keyboard(Key::LShift),
            orbit_speed: 0.05,
            pan_speed: 0.1,
            zoom_speed: 0.1,
        }
    }

}

///
/// A 3dsMax / Blender-style camera that orbits around a target point
/// 
pub struct OrbitZoomCamera<T=f32> {

    /// origin of camera rotation
    pub target: Vector3<T>,

    /// Rotation of camera relative to world-space positive z-axis
    pub rotation: Quaternion<T>,

    /// Pitch up/down from target
    pub pitch: T,
    
    /// Yaw left/right from target
    pub yaw: T,

    /// camera distance from target
    pub distance: T,

    /// Settings for the camera
    pub settings: OrbitZoomCameraSettings<T>,

    /// Current keys that are pressed
    keys: Keys,
}


impl OrbitZoomCamera {

    ///
    /// Create a new OrbitZoomCamera targeting the given coordinates
    /// 
    pub fn new(target: [f32; 3], settings: OrbitZoomCameraSettings) -> OrbitZoomCamera {
        OrbitZoomCamera { 
            target: Vector3::new(target[0], target[1], target[2]),
            rotation: Quaternion::identity(),
            distance: 10.0,
            pitch: 0f32,
            yaw: 0f32,
            keys: Keys::empty(),
            settings: settings
        }
    }

    ///
    /// Return a Camera for the current OrbitZoomCamera configuration
    ///
    pub fn camera(&self, dt: f64) -> Camera<f32> {
        let target_to_camera = self.rotation.rotate_vector(&Vector3::new(0.0, 0.0, self.distance));
        let mut camera = Camera::new([self.target[0] + target_to_camera.x,
                                      self.target[1] + target_to_camera.y,
                                      self.target[2] + target_to_camera.z]);
        camera.set_rotation(&self.rotation);
        camera
    }

    ///
    /// Orbit the camera using the given horizontal and vertical params,
    /// or zoom or pan if the appropriate modifier keys are pressed
    ///
    fn control_camera(&mut self, dx: f32, dy: f32) {
        if self.keys.contains(PAN) {

            // Pan target position along plane normal to camera direction
            let dx = dx * self.settings.pan_speed;
            let dy = dy * self.settings.pan_speed;
            let right = self.rotation.rotate_vector(&Vector3::unit_x());
            let up = self.rotation.rotate_vector(&Vector3::unit_y());
            self.target = self.target + up.mul_s(dy) + right.mul_s(dx);

        } else if self.keys.contains(ZOOM) {

            // Zoom to / from target
            self.distance = self.distance + dy * self.settings.zoom_speed;

        } else {

            // Orbit around target
            let dx = dx * self.settings.orbit_speed;
            let dy = dy * self.settings.orbit_speed;

            self.yaw = self.yaw + dx;
            self.pitch = self.pitch + dy;
            self.rotation = Rotation3::from_euler(-rad(self.yaw), rad(0f32), rad(self.pitch));
        }
    }

    ///
    /// Respond to scroll and key press/release events
    /// 
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

        use event::{ MouseRelativeEvent, MouseScrollEvent, PressEvent, ReleaseEvent };
        use input::keyboard::Key;
        use input::Button::Keyboard;

        e.mouse_scroll(|dx, dy| {
            self.control_camera(dx as f32, dy as f32);
        });

        e.mouse_relative(|dx, dy| {
            if self.keys.contains(ORBIT){
                self.control_camera(-dx as f32, dy as f32);
            }
        });

        e.press(|button| {
            match button {
                x if x == self.settings.orbit_button => self.keys.insert(ORBIT),
                x if x == self.settings.pan_button => self.keys.insert(PAN),
                x if x == self.settings.zoom_button => self.keys.insert(ZOOM),
                _ => {}
            }
        });

        e.release(|button| {
            match button {
                x if x == self.settings.orbit_button => self.keys.remove(ORBIT),
                x if x == self.settings.pan_button => self.keys.remove(PAN),
                x if x == self.settings.zoom_button => self.keys.remove(ZOOM),
                _ => {}
            }
        });
    }
}


