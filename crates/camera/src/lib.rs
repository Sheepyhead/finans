#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,clippy::redundant_field_names
)]
use bevy::{prelude::*, render::camera::ScalingMode};

pub struct Camera;

impl Plugin for Camera {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 15.0 / 12.0;

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 7.5 * RESOLUTION;
    camera.orthographic_projection.left = -7.5 * RESOLUTION;

    camera.orthographic_projection.top = 7.5;
    camera.orthographic_projection.bottom = -7.5;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
