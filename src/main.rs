use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

enum BallDirection {
    RIGHT,
    LEFT,
}

#[derive(Component)]
struct Ball {
    velocity: f32,
    direction: BallDirection,
}

static MAX_WIDTH: f32 = 400.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_ball)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut matierals: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    let ball_shape = Mesh2dHandle(meshes.add(Circle { radius: 5.0 }));
    let color = Color::rgb(255., 255., 255.);
    commands.spawn((
        Ball {
            velocity: 2.5,
            direction: BallDirection::RIGHT,
        },
        MaterialMesh2dBundle {
            mesh: ball_shape,
            material: matierals.add(color),
            ..default()
        },
    ));
}

fn move_ball(_time: Res<Time>, mut query: Query<(&mut Ball, &mut Transform)>) {
    let (mut ball, mut transform) = query.single_mut();

    if transform.translation.x >= MAX_WIDTH {
        ball.direction = BallDirection::LEFT;
    } else if transform.translation.x <= MAX_WIDTH * -1. {
        ball.direction = BallDirection::RIGHT;
    }

    match ball.direction {
        BallDirection::RIGHT => transform.translation.x += ball.velocity,
        BallDirection::LEFT => transform.translation.x -= ball.velocity,
    }
}
