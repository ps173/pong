use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const MAX_WIDTH: f32 = 400.;
const MAX_HEIGHT: f32 = 300.;
// x coordinates
// const LEFT_WALL: f32 = -400.;
// const RIGHT_WALL: f32 = 400.;

// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_THICKNESS: f32 = 20.;
const WALL_COLOR: Color = Color::rgb(255., 255., 255.);
const PADDLE_WALL_GAP: f32 = 5.;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    radius: f32,
}

#[derive(Component)]
struct Paddle {
    width: f32,
    height: f32,
    velocity: f32,
}

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Bottom,
    Top,
}

impl WallLocation {
    /// Location of the *center* of the wall, used in `transform.translation()`
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    /// (x, y) dimensions of the wall, used in `transform.scale()`
    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        // FIXME: MAX_WIDTH is half of screen resolution;
        let arena_width = MAX_WIDTH * 2.;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

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
        .add_systems(Startup, (setup, spawn_balls, spawn_paddles))
        .add_systems(
            FixedUpdate,
            (handle_collision, move_ball, move_paddle, reset_ball),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut matierals: ResMut<Assets<ColorMaterial>>,
) {
    let ball = Ball {
        velocity: Vec2::new(2.5, 1.),
        radius: 5.,
    };
    let ball_shape = Mesh2dHandle(meshes.add(Circle {
        radius: ball.radius,
    }));
    let color = Color::rgb(255., 255., 255.);
    commands.spawn((
        ball,
        MaterialMesh2dBundle {
            mesh: ball_shape,
            material: matierals.add(color),
            ..default()
        },
    ));
}

fn spawn_paddles(mut commands: Commands) {
    let paddle = Paddle {
        width: 5.,
        height: 50.,
        velocity: 5.,
    };
    let paddle_width = paddle.width;
    let color = Color::rgb(255., 255., 255.);

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(MAX_WIDTH - paddle_width - PADDLE_WALL_GAP, 0., 0.0),
                scale: Vec3::new(paddle.width, paddle.height, 0.),
                ..default()
            },
            sprite: Sprite { color, ..default() },
            ..default()
        },
        paddle,
        Collider,
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    let (paddle, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        if transform.translation.y <= -MAX_HEIGHT + WALL_THICKNESS {
            transform.translation.y -= 0.;
        } else {
            transform.translation.y -= paddle.velocity;
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        if transform.translation.y >= MAX_HEIGHT - WALL_THICKNESS {
            transform.translation.y += 0.;
        } else {
            transform.translation.y += paddle.velocity;
        }
    }
}

fn handle_collision(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (mut ball, ball_transform) = ball_query.single_mut();

    for transform in &collider_query {
        let collision = collide_with_side(
            BoundingCircle::new(ball_transform.translation.truncate(), ball.radius),
            Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Right | Collision::Left => {
                    println!("collided: {:?}", collision);
                    reflect_x = true
                }
                Collision::Top | Collision::Bottom => {
                    println!("collided: {:?}", collision);
                    reflect_y = true
                }
            }

            if reflect_x {
                ball.velocity.x = -ball.velocity.x
            }
            if reflect_y {
                ball.velocity.y = -ball.velocity.y
            }
        }
    }
}

fn reset_ball(mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    let (mut ball, mut transform) = ball_query.single_mut();

    if transform.translation.x >= MAX_WIDTH {
        transform.translation.x = 0.;
        ball.velocity.x = ball.velocity.x.abs()
    }
    if transform.translation.x <= -MAX_WIDTH {
        transform.translation.x = 0.;
        ball.velocity.x = ball.velocity.x.abs()
    }
}

fn move_ball(_time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    let (ball, mut transform) = query.single_mut();

    transform.translation.x += ball.velocity.x;
    transform.translation.y += ball.velocity.y;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
