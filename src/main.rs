use bevy::{prelude::*, render::view::visibility};
use bevy::time::FixedTimestep;
use bevy_flycam::{MovementSettings, PlayerPlugin};

pub mod lib;
use lib::{Block, Grid};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

const BLOCK_SIZE: f32 = 1.0;
const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 15;
const GRID_DEPTH: usize = 15;

fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT, GRID_DEPTH, BLOCK_SIZE);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(grid)
        .add_startup_system(spawn_scene)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy 3d playground".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01))
                .with_system(update_grid)
        )
        .run();
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<Grid>,
) {
    grid.generate_blocks();

    for block in grid.grid.values() {
        commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: block.size })),
            material: materials.add(StandardMaterial {
                base_color: block.color,
                double_sided: true,
                ..Default::default()
            }),
            visibility: Visibility { 
                is_visible: block.is_alive(),
            },
            transform: Transform::from_translation(Vec3::new(
                block.position[0] as f32,
                block.position[1] as f32,
                block.position[2] as f32,
            )),
            ..default()
        })
        .insert(Block { ..block.to_owned() });
    }

    //MIDDLE POINT
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });

    //LEFT POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 + 5.0,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });

    //RIGHT POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            -5.0,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });

    //TOP POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 + 5.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });

    //BOTTOM POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            -5.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });

    //FRONT POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 + 5.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });

    //BACK POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 / 2.0,
            -5.0,
        )),
        point_light: PointLight {
            intensity: 24000.0,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: 50.0,
            ..Default::default()
        },
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 20.5, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn despawn_system<M: Component>(mut commands: Commands, query: Query<Entity, With<M>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

fn update_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<Grid>,
    mut blocks: Query<(&Block, &mut Visibility)>,
) {
    grid.update();

    // loop through all blocks and update their visibility if they are alive or dead

    for (block, mut visibility) in blocks.iter_mut() {
        if let Some(new_block) = grid.grid.get(&block.position) {
            visibility.is_visible = new_block.is_alive();
        }
    }
}

