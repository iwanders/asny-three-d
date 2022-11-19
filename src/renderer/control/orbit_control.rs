use super::*;
use crate::core::*;

///
/// A control that makes the camera orbit around a target.
///
pub struct OrbitControl {
    control: CameraControl,
    new_target: Option<Vec3>,
}

impl OrbitControl {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft { speed: 0.5 },
                left_drag_vertical: CameraAction::OrbitUp { speed: 0.5 },
                scroll_vertical: CameraAction::Zoom {
                    min: min_distance,
                    max: max_distance,
                    speed: 0.1,
                },
                ..Default::default()
            },
            new_target: Some(target),
        }
    }
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target,
    /// the target can be moved with the right mouse button.
    pub fn new_with_pan(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft { speed: 0.5 },
                left_drag_vertical: CameraAction::OrbitUp { speed: 0.5 },
                right_drag_horizontal: CameraAction::OrbitTargetLeft { speed: 0.5 },
                right_drag_vertical: CameraAction::OrbitTargetUp { speed: 0.5 },
                scroll_vertical: CameraAction::Zoom {
                    min: min_distance,
                    max: max_distance,
                    speed: 0.1,
                },
                ..Default::default()
            },
            new_target: Some(target),
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        if let Some(new_target) = self.new_target.take() {
            let position = camera.position().clone();
            let up = camera.up().clone();
            camera.set_view(position, new_target, up);
        }
        if let CameraAction::Zoom { speed, min, max } = &mut self.control.scroll_vertical {
            let x = camera.target().distance(*camera.position());
            *speed = 0.5 * smoothstep(*min, *max, x) + 0.001;
        }
        self.control.handle_events(camera, events)
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).max(0.0).min(1.0);
    t * t * (3.0 - 2.0 * t)
}
