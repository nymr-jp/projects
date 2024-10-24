#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle},
};

const STAGE: [[usize; 100]; 10] = [
    [0; 100],
    [0; 100],
    [0; 100],
    [0; 100],
    [0; 100],
    [0; 100],
    [0; 100],
    [0; 100],
    [0; 100],
    [1; 100]
];

enum Collision {
    Left,
    Right,
    Up,
    Down
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (collision_check, move_player).chain());
    app.run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct Body {
    height: f32,
    width: f32
}

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Collider;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(10.0, 30.0)).into(),
            material: materials.add(Color::srgb(1.0, 0.5, 0.5)),
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 1.0)),
            ..default()
        },
        Player,
        Velocity(0., 0.),
        Body {
            height: 10.,
            width: 10.
        },
        Collider
    ));

    for (row_index, row) in STAGE.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if *column != 0 {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Rectangle::new(10.0, 10.0)).into(),
                        material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
                        transform: Transform::from_translation(
                            Vec3::new(
                                10.0 * (column_index as f32) - 500.0,
                                10.0 * (row_index as f32) - 400.0,
                                1.0
                            )),
                        ..default()
                    },
                    Block,
                    Body {
                        height: 10.,
                        width: 10.
                    },
                    Velocity(0., 0.),
                    Collider
                ));
            }
        }
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;
    let mut vertical_direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        vertical_direction += 1.0;
    }

    paddle_transform.translation.x = paddle_transform.translation.x + direction * 1000. * time.delta_seconds();
    paddle_transform.translation.y = paddle_transform.translation.y + vertical_direction * 1000. * time.delta_seconds();
}

fn collision_check(
    query: Query<(&Transform, &Body, &Velocity), (With<Collider>)>,
) {
    for (transform1, body1, velocity1) in &query {
        for (transform2, body2, velocity2) in &query {
            transform1.x + body1.width / 2.
        }
    }
}
