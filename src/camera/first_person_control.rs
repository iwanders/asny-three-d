use crate::camera::*;
use crate::frame::*;
use crate::Error;

pub struct FirstPersonControl {
    control: CameraControl,
}

impl FirstPersonControl {
    pub fn new(speed: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::Yaw {
                    speed: std::f32::consts::PI / 1800.0,
                },
                scroll_vertical: CameraAction::Forward { speed },
                ..Default::default()
            },
        }
    }

    pub fn handle_events(
        &mut self,
        camera: &mut Camera,
        events: &mut [Event],
    ) -> Result<bool, Error> {
        self.control.handle_events(camera, events)
    }
}