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

use bevy::{prelude::*, window::PresentMode};
use board::Board;
use camera::Camera;
#[cfg(debug_assertions)]
use debug::Debug;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 15.0 / 12.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Finans".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .add_startup_system(setup_board)
        .run();
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        #[cfg(debug_assertions)]
        group.add(Debug);
        group.add(Camera);
    }
}

const BASIC_SPACE_SIZE: f32 = 1.3;
const SPECIAL_SPACE_SIZE: f32 = 1.5;

fn setup_board(mut commands: Commands) {
    commands.insert_resource(Board::new(1, 46));

    let board = commands
        .spawn_bundle((
            GameBoard,
            Transform::default(),
            GlobalTransform::default(),
            Name::new("Board"),
        ))
        .id();

    for (i, space) in (1..=9).enumerate() {
        let space = spawn_basic_board_space(
            &mut commands,
            Vec3::new(
                -8.16 - BASIC_SPACE_SIZE / 2.0,
                i as f32 * BASIC_SPACE_SIZE // position based on index
                    - 4.0 * BASIC_SPACE_SIZE, // offset by half the length of the column
                0.0,
            ),
            if i % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            },
            space,
        );

        commands.entity(board).add_child(space);
    }

    for (i, space) in (11..=22).enumerate() {
        let space = spawn_basic_board_space(
            &mut commands,
            Vec3::new(
                i as f32 * BASIC_SPACE_SIZE // position based on index
                - 6.0 * BASIC_SPACE_SIZE // offset by half the length of the column
                    + BASIC_SPACE_SIZE / 2.0,
                7.5 - BASIC_SPACE_SIZE / 2.0,
                0.0,
            ),
            if i % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            },
            space,
        );

        commands.entity(board).add_child(space);
    }

    for (i, space) in (24..=32).rev().enumerate() {
        let space = spawn_basic_board_space(
            &mut commands,
            Vec3::new(
                8.16 + BASIC_SPACE_SIZE / 2.0,
                i as f32 * BASIC_SPACE_SIZE // position based on index
                    - 4.0 * BASIC_SPACE_SIZE, // offset by half the length of the column
                0.0,
            ),
            if i % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            },
            space,
        );

        commands.entity(board).add_child(space);
    }

    for (i, space) in (34..=45).rev().enumerate() {
        let space = spawn_basic_board_space(
            &mut commands,
            Vec3::new(
                i as f32 * BASIC_SPACE_SIZE // position based on index
                - 6.0 * BASIC_SPACE_SIZE
                    + BASIC_SPACE_SIZE / 2.0, // offset by half the length of the column
                -7.5 + BASIC_SPACE_SIZE / 2.0,
                0.0,
            ),
            if i % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            },
            space,
        );

        commands.entity(board).add_child(space);
    }

    let special_spaces = [
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(SPECIAL_SPACE_SIZE)),
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform::from_xyz(-8.6, -7.5 + SPECIAL_SPACE_SIZE / 2.0, 0.0),
                ..default()
            })
            .insert_bundle((BoardSpace(0), Name::new("Space 0")))
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(SPECIAL_SPACE_SIZE)),
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform::from_xyz(-8.6, 7.5 - SPECIAL_SPACE_SIZE / 2.0, 0.0),
                ..default()
            })
            .insert_bundle((BoardSpace(0), Name::new("Space 10")))
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(SPECIAL_SPACE_SIZE)),
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform::from_xyz(8.6, 7.5 - SPECIAL_SPACE_SIZE / 2.0, 0.0),
                ..default()
            })
            .insert_bundle((BoardSpace(0), Name::new("Space 23")))
            .id(),
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(SPECIAL_SPACE_SIZE)),
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform::from_xyz(8.6, -7.5 + SPECIAL_SPACE_SIZE / 2.0, 0.0),
                ..default()
            })
            .insert_bundle((BoardSpace(0), Name::new("Space 33")))
            .id(),
    ];

    commands.entity(board).push_children(&special_spaces);
}

fn spawn_basic_board_space(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    space: u32,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(BASIC_SPACE_SIZE)),
                color,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        })
        .insert_bundle((BoardSpace(space), Name::new(format!("Space {}", space))))
        .id()
}

#[derive(Component)]
struct GameBoard;

#[derive(Component)]
struct BoardSpace(u32);
