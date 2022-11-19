use super::*;
use crate::core::*;

///
/// A set of possible actions to apply to a camera when recieving input.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraAction {
    /// No action.
    None,
    /// Rotate the camera around the horizontal axis as seen from the camera.
    Pitch {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Orbits around the given target in the up direction as seen from the camera.
    OrbitUp {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Move the target of the orbit upwards as seen from the camera.
    OrbitTargetUp {
        /// The speed of the translation.
        speed: f32,
    },
    /// Rotate the camera around the vertical axis as seen from the camera.
    Yaw {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Orbits around the given target in the left direction as seen from the camera.
    OrbitLeft {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Move the target of the orbit to the left as seen from the camera.
    OrbitTargetLeft {
        /// The speed of the translation.
        speed: f32,
    },
    /// Rotate the camera around the forward axis as seen from the camera.
    Roll {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Moves the camera to the left.
    Left {
        /// The speed of the translation.
        speed: f32,
    },
    /// Moves the camera up.
    Up {
        /// The speed of the translation.
        speed: f32,
    },
    /// Moves the camera forward.
    Forward {
        /// The speed of the translation.
        speed: f32,
    },
    /// Zooms towards the given target.
    Zoom {
        /// The speed of the zoom.
        speed: f32,
        /// The minimum distance to the target.
        min: f32,
        /// The maximum distance to the target.
        max: f32,
    },
}

impl std::default::Default for CameraAction {
    fn default() -> Self {
        Self::None
    }
}

///
/// A customizable controller for the camera.
/// It is possible to specify a [CameraAction] for each of the input events.
///
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraControl {
    /// Specifies what happens when dragging horizontally with the left mouse button.
    pub left_drag_horizontal: CameraAction,
    /// Specifies what happens when dragging vertically with the left mouse button.
    pub left_drag_vertical: CameraAction,
    /// Specifies what happens when dragging horizontally with the middle mouse button.
    pub middle_drag_horizontal: CameraAction,
    /// Specifies what happens when dragging vertically with the middle mouse button.
    pub middle_drag_vertical: CameraAction,
    /// Specifies what happens when dragging horizontally with the right mouse button.
    pub right_drag_horizontal: CameraAction,
    /// Specifies what happens when dragging vertically with the right mouse button.
    pub right_drag_vertical: CameraAction,
    /// Specifies what happens when scrolling horizontally.
    pub scroll_horizontal: CameraAction,
    /// Specifies what happens when scrolling vertically.
    pub scroll_vertical: CameraAction,
}

impl CameraControl {
    /// Creates a new default CameraControl.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion {
                    delta,
                    button,
                    handled,
                    ..
                } => {
                    if !*handled && button.is_some() {
                        if let Some(b) = button {
                            let (control_horizontal, control_vertical) = match b {
                                MouseButton::Left => {
                                    (self.left_drag_horizontal, self.left_drag_vertical)
                                }
                                MouseButton::Middle => {
                                    (self.middle_drag_horizontal, self.middle_drag_vertical)
                                }
                                MouseButton::Right => {
                                    (self.right_drag_horizontal, self.right_drag_vertical)
                                }
                            };
                            *handled = self.handle_action(camera, control_horizontal, delta.0);
                            *handled |= self.handle_action(camera, control_vertical, delta.1);
                            change |= *handled;
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        *handled = self.handle_action(camera, self.scroll_horizontal, delta.0);
                        *handled |= self.handle_action(camera, self.scroll_vertical, delta.1);
                        change |= *handled;
                    }
                }
                _ => {}
            }
        }
        change
    }

    fn handle_action(&mut self, camera: &mut Camera, control_type: CameraAction, x: f64) -> bool {
        match control_type {
            CameraAction::Pitch { speed } => {
                camera.pitch(radians(speed * x as f32));
            }
            CameraAction::OrbitUp { speed } => {
                let target = camera.target().clone();
                camera.rotate_around_with_fixed_up(&target, 0.0, speed * x as f32);
            }
            CameraAction::OrbitTargetUp { speed } => {
                // Calculate the change, allowing translation orthogonal to the current view and
                // the right direction, which is modified by OrbitTargetLeft.
                let view = camera.view_direction().clone();
                let neg_right = -camera.right_direction().clone();
                let change = x as f32 * (speed * view.cross(neg_right));

                // Update the positions, moving both the target and the camera based on the pan.
                let up = camera.up().clone();
                let new_target = camera.target().clone() + change;
                let position = camera.position().clone() + change;

                camera.set_view(position, new_target, up);
            }
            CameraAction::Yaw { speed } => {
                camera.yaw(radians(speed * x as f32));
            }
            CameraAction::OrbitLeft { speed } => {
                let target = camera.target().clone();
                camera.rotate_around_with_fixed_up(&target, speed * x as f32, 0.0);
            }
            CameraAction::OrbitTargetLeft { speed } => {
                // Calculate the change by multiplying the right direction of the camera with the
                // speed and change.
                let right = camera.right_direction();
                let change = -x as f32 * right * speed;

                // Update the positions, moving both the target and the camera based on the pan.
                let new_target = camera.target().clone() + change;
                let position = camera.position().clone() + change;
                let up = camera.up().clone();

                camera.set_view(position, new_target, up);
            }
            CameraAction::Roll { speed } => {
                camera.roll(radians(speed * x as f32));
            }
            CameraAction::Left { speed } => {
                let change = -camera.right_direction() * x as f32 * speed;
                camera.translate(&change);
            }
            CameraAction::Up { speed } => {
                let right = camera.right_direction();
                let up = right.cross(camera.view_direction());
                let change = up * x as f32 * speed;
                camera.translate(&change);
            }
            CameraAction::Forward { speed } => {
                let change = camera.view_direction() * speed * x as f32;
                camera.translate(&change);
            }
            CameraAction::Zoom { speed, min, max } => {
                let target = camera.target().clone();
                camera.zoom_towards(&target, speed * x as f32, min, max);
            }
            CameraAction::None => {}
        }
        control_type != CameraAction::None
    }
}
