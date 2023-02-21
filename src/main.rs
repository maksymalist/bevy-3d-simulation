use bevy::input::ButtonState;
use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy::time::FixedTimestep;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy::input::mouse::{MouseButton, MouseMotion};

pub mod lib;
use lib::{Block, Grid};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

const BLOCK_SIZE: f32 = 1.0;
const GRID_WIDTH: usize = 24;
const GRID_HEIGHT: usize = 24;
const GRID_DEPTH: usize = 24;


#[derive(Resource)]
struct CursorSettings {
    visibility: bool,
}

fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT, GRID_DEPTH, BLOCK_SIZE);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(grid)
        .insert_resource(CursorSettings {
            visibility: true,
        })
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
        .add_plugin(EguiPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.05))
                .with_system(update_grid),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01))
                .with_system(camera_movement),
        )
        .run();
}

fn ui_example_system(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
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
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: block.size / 2.0,
                    subdivisions: 1,
                })),
                // mesh: meshes.add(Mesh::from(shape::Cube { size: block.size })),
                material: materials.add(StandardMaterial {
                    base_color: block.color,
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

    const LIGHT_INTENSITY: f32 = 24000.0;
    const LIGHT_RANGE: f32 = 100.0;
    const LIGHT_MARGIN: f32 = 15.0;

    //MIDDLE POINT
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
            ..Default::default()
        },
        ..default()
    });

    //LEFT POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 + LIGHT_MARGIN,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
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
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
            ..Default::default()
        },
        ..default()
    });

    //TOP POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 + LIGHT_MARGIN,
            GRID_DEPTH as f32 / 2.0,
        )),
        point_light: PointLight {
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
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
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
            ..Default::default()
        },
        ..default()
    });

    //FRONT POINT

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(
            GRID_WIDTH as f32 / 2.0,
            GRID_HEIGHT as f32 / 2.0,
            GRID_DEPTH as f32 + LIGHT_MARGIN,
        )),
        point_light: PointLight {
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
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
            intensity: LIGHT_INTENSITY,
            color: Color::rgb(1.0, 1.0, 1.0),
            range: LIGHT_RANGE,
            ..Default::default()
        },
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(
            20.0 + GRID_WIDTH as f32,
            20.5 + GRID_HEIGHT as f32,
            50.0 + GRID_DEPTH as f32,
        )
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Camera)>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut mouse_button: ResMut<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut settings: ResMut<CursorSettings>,
) {

    for (mut transform, mut camera) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // FORWARD

        if keyboard_input.pressed(KeyCode::W) {
            // make the 3d camera move forward in the direction it is facing
            let forward = transform.clone().forward();
            transform.translation += forward;
        }
        // BACKWARD

        if keyboard_input.pressed(KeyCode::S) {
            // make the 3d camera move backward in the direction it is facing
            let forward = transform.clone().forward();
            transform.translation -= forward;
        }

        // LEFT

        if keyboard_input.pressed(KeyCode::A) {
            // make the 3d camera move left in the direction it is facing
            let forward = transform.clone().forward();
            transform.translation -= forward.cross(Vec3::Y);
        }

        // RIGHT

        if keyboard_input.pressed(KeyCode::D) {
            // make the 3d camera move right in the direction it is facing
            let forward = transform.clone().forward();
            transform.translation += forward.cross(Vec3::Y);
        }

        // UP

        if keyboard_input.pressed(KeyCode::Space) {
            transform.translation = Vec3::new(transform.translation.x, transform.translation.y + 1.0, transform.translation.z);
        }

        // DOWN

        if keyboard_input.pressed(KeyCode::LShift) {
            transform.translation = Vec3::new(transform.translation.x, transform.translation.y - 1.0, transform.translation.z);
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            settings.visibility = !settings.visibility;
            if let Some(window) = windows.get_primary_mut() {
                window.set_cursor_visibility(settings.visibility);
            }
        }

        // MOUSE

        if let Some(window) = windows.get_primary_mut() {
            if mouse_button.pressed(MouseButton::Left) {
                for event in mouse_motion.get_reader().iter(&mouse_motion) {
                    let delta = event.delta;
                    let sensitivity = 0.0015;
                    let delta = Vec2::new(delta.x as f32, delta.y as f32) * sensitivity;
                    transform.rotation *= Quat::from_rotation_y(-delta.x);
                    transform.rotation *= Quat::from_rotation_x(-delta.y);
                }
            }
        }

    }
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