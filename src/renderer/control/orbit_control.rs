use super::*;
use crate::core::*;

///
/// A control that makes the camera orbit around a target.
///
pub struct OrbitControl {
    control: CameraControl,
    new_target: Option<Vec3>,
}

/// Configuration struct to specify the (initial) properties of the OrbitControl.
pub struct OrbitControlConfig {
    /// The target to orbit around.
    pub target: Vec3,
    /// Minimum distance from target.
    pub min_distance: f32,
    /// Maximum distance from target.
    pub max_distance: f32,
    /// Speed of horizontal rotation around target.
    pub speed_orbit_horizontal: f32,
    /// Speed of vertical rotation around target.
    pub speed_orbit_vertical: f32,
    /// If set, speed of panning the target left-right of current view, disabled if None.
    pub speed_orbit_target_left: Option<f32>,
    /// If set, speed of panning the target top-bottom of current view, disabled if None.
    pub speed_orbit_target_up: Option<f32>,
    /// The speed at which to zoom towards the target.
    pub speed_zoom: f32,
}

impl Default for OrbitControlConfig {
    fn default() -> Self {
        OrbitControlConfig {
            target: Vec3::new(0.0, 0.0, 0.0),
            min_distance: 1.0,
            max_distance: 100.0,
            speed_orbit_horizontal: 0.5,
            speed_orbit_vertical: 0.5,
            speed_orbit_target_left: None,
            speed_orbit_target_up: None,
            speed_zoom: 0.1,
        }
    }
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

    /// Create a new orbit configuration with the provided configuration.
    pub fn new_with_config(config: OrbitControlConfig) -> Self {
        let right_drag_horizontal = config
            .speed_orbit_target_left
            .map_or(CameraAction::None, |v| CameraAction::OrbitTargetLeft {
                speed: v,
            });
        let right_drag_vertical = config
            .speed_orbit_target_left
            .map_or(CameraAction::None, |v| CameraAction::OrbitTargetUp {
                speed: v,
            });
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft {
                    speed: config.speed_orbit_horizontal,
                },
                left_drag_vertical: CameraAction::OrbitUp {
                    speed: config.speed_orbit_vertical,
                },
                right_drag_horizontal,
                right_drag_vertical,
                scroll_vertical: CameraAction::Zoom {
                    min: config.min_distance,
                    max: config.max_distance,
                    speed: config.speed_zoom,
                },
                ..Default::default()
            },
            new_target: Some(config.target),
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
