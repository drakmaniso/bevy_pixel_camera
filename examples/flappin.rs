use bevy::{
    core::FixedTimestep,
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelSpriteQuad};

// GAME CONSTANTS /////////////////////////////////////////////////////////////

const WIDTH: f32 = 240.0;
const HEIGHT: f32 = 240.0;
const LEFT: f32 = -WIDTH / 2.0;
const RIGHT: f32 = LEFT + WIDTH;
const BOTTOM: f32 = -HEIGHT / 2.0;
const _TOP: f32 = BOTTOM + HEIGHT;

const CLOUD_WIDTH: f32 = 64.0;
const CLOUD_HEIGHT: f32 = 18.0;

const PILLAR_WIDTH: f32 = 19.0;
const PILLAR_HEIGHT: f32 = 480.0;
const PILLAR_SPACING: f32 = 80.0;
const PILLAR_GAP: f32 = 70.0;
const PILLAR_RANGE: f32 = 100.0;

const BIRD_X: f32 = -80.0;
const BIRD_DX: f32 = 14.0;
const BIRD_DY: f32 = 10.0;
const BIRD_RADIUS: f32 = 8.0;

const FRAME: f64 = 1.0 / 60.0;

const FALLING_JERK: f32 = -2300.0;
const FLAP_VELOCITY: f32 = 100.0;
const FLAP_ACCELERATION: f32 = 90.0;

// SETUP //////////////////////////////////////////////////////////////////////

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
    StartScreen,
    Playing,
    GameOver,
}

fn main() {
    App::build()
        .add_state(GameState::StartScreen)
        .insert_resource(WindowDescriptor {
            title: "Flappin'".to_string(),
            width: 720.0,
            height: 720.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelCameraPlugin)
        .insert_resource(Rng { mz: 0, mw: 0 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Timer::from_seconds(0.25, false))
        .add_startup_system(setup.system().label("setup"))
        .add_startup_system(spawn_bird.system().after("setup"))
        .add_startup_system(spawn_clouds.system().after("setup"))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system_set(
            SystemSet::on_update(GameState::StartScreen)
                .with_system(animate_flying_bird.system())
                .with_system(press_to_start.system()),
        )
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_pillars.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(animate_flappin_bird.system())
                .with_system(flap.system())
                .with_system(collision_detection.system()),
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over.system()))
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(press_to_start.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(despawn_pillars.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 * FRAME))
                .with_system(animate_pillars.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0 * FRAME))
                .with_system(animate_clouds.system()),
        )
        .run();
}

fn setup(mut commands: Commands, time: Res<Time>, mut rng: ResMut<Rng>) {
    *rng = Rng {
        mz: time.startup().elapsed().as_secs() as u32,
        mw: 678,
    };

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        WIDTH as i32,
        HEIGHT as i32,
    ));
}

// START SCREEN ///////////////////////////////////////////////////////////////

fn press_to_start(
    mut key_events: EventReader<KeyboardInput>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut birds: Query<(&mut Transform, &mut BirdPhysics), With<Bird>>,
) {
    timer.tick(time.delta());
    if timer.finished()
        && key_events
            .iter()
            .any(|ev| ev.state == ElementState::Pressed)
    {
        for (mut transform, mut physics) in birds.iter_mut() {
            *transform = Transform::from_xyz(BIRD_X, 0.0, 1.0);
            physics.velocity = FLAP_VELOCITY;
            physics.acceleration = FLAP_ACCELERATION;
        }
        state
            .set(GameState::Playing)
            .expect("Problem while switching to playing state");
    }
}

// THE BIRD ///////////////////////////////////////////////////////////////////

// Component
struct Bird;

// Component
struct BirdPhysics {
    velocity: f32,
    acceleration: f32,
}

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    quad: Res<PixelSpriteQuad>,
) {
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("flappin-bird.png"),
        Vec2::new(26.0, 21.0),
        4,
        1,
    ));
    commands
        .spawn()
        .insert(Bird)
        .insert(BirdPhysics {
            velocity: 100.0,
            acceleration: 0.0,
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_translation(Vec3::new(BIRD_X, 0.0, 1.0)),
            mesh: quad.clone().into(),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.150, true));
}

fn animate_flying_bird(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite), With<Bird>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = ((sprite.index as usize + 1) % 3) as u32;
        }
    }
}

fn animate_flappin_bird(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BirdPhysics, &mut TextureAtlasSprite), With<Bird>>,
) {
    for (mut transform, mut physics, mut sprite) in query.iter_mut() {
        let dt = time.delta().as_secs_f32();
        let y = (transform.translation.y + physics.velocity * dt).round();
        *transform = Transform::from_xyz(BIRD_X, y, 1.0);
        physics.velocity += physics.acceleration * dt;
        physics.acceleration += FALLING_JERK * dt;
        sprite.index = if physics.acceleration < -1200.0 {
            2
        } else if physics.acceleration > -300.0 {
            0
        } else {
            1
        }
    }
}

fn flap(
    mut key_events: EventReader<KeyboardInput>,
    mut birds: Query<&mut BirdPhysics, With<Bird>>,
) {
    if key_events
        .iter()
        .any(|ev| ev.state == ElementState::Pressed)
    {
        for mut physics in birds.iter_mut() {
            physics.velocity = FLAP_VELOCITY;
            physics.acceleration = FLAP_ACCELERATION;
        }
    }
}

fn game_over(mut timer: ResMut<Timer>, mut birds: Query<&mut TextureAtlasSprite, With<Bird>>) {
    timer.reset();
    for mut sprite in birds.iter_mut() {
        sprite.index = 3;
    }
}

fn collision_detection(
    mut state: ResMut<State<GameState>>,
    birds: Query<&Transform, With<Bird>>,
    pillars: Query<&Transform, With<Pillar>>,
) {
    for bird_transform in birds.iter() {
        let bird_x = bird_transform.translation.x + BIRD_DX;
        let bird_y = bird_transform.translation.y + BIRD_DY;
        let collides = pillars.iter().any(|&pillar_transform| {
            let pillar_x = pillar_transform.translation.x;
            let pillar_y = pillar_transform.translation.y + PILLAR_HEIGHT / 2.0;
            circle_box_collide(
                bird_x,
                bird_y,
                BIRD_RADIUS,
                pillar_x,
                pillar_x + PILLAR_WIDTH,
                pillar_y + PILLAR_GAP / 2.0,
                pillar_y + PILLAR_HEIGHT,
            ) || circle_box_collide(
                bird_x,
                bird_y,
                BIRD_RADIUS,
                pillar_x,
                pillar_x + PILLAR_WIDTH,
                pillar_y - PILLAR_HEIGHT,
                pillar_y - PILLAR_GAP / 2.0,
            )
        });
        if collides || bird_y < BOTTOM {
            state
                .set(GameState::GameOver)
                .expect("Problem while switching to game over state");
        }
    }
}

// THE PILLARS ////////////////////////////////////////////////////////////////

// Component
struct Pillar;

fn spawn_pillars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    quad: Res<PixelSpriteQuad>,
    mut rng: ResMut<Rng>,
) {
    let atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("flappin-pillars.png"),
        Vec2::new(PILLAR_WIDTH, PILLAR_HEIGHT),
        1,
        1,
    ));

    let mut x = RIGHT;
    while x < RIGHT + WIDTH + PILLAR_SPACING {
        let y = (rng.rand_range(0..PILLAR_RANGE as u32) as f32 - PILLAR_RANGE / 2.0).round();
        commands
            .spawn()
            .insert(Pillar)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: atlas.clone(),
                transform: Transform::from_xyz(x, (y - PILLAR_HEIGHT / 2.0).round(), 2.0),
                mesh: quad.clone().into(),
                ..Default::default()
            });
        x += PILLAR_SPACING;
    }
}

fn animate_pillars(
    mut rng: ResMut<Rng>,
    state: Res<State<GameState>>,
    mut query: Query<&mut Transform, With<Pillar>>,
) {
    if *state.current() == GameState::GameOver {
        return;
    }
    for mut transform in query.iter_mut() {
        *transform = transform.mul_transform(Transform::from_xyz(-1.0, 0.0, 0.0));
        if transform.translation.x + PILLAR_SPACING < LEFT {
            let y = (rng.rand_range(0..PILLAR_RANGE as u32) as f32 - PILLAR_RANGE / 2.0).round();
            *transform = Transform::from_xyz(RIGHT, (y - PILLAR_HEIGHT / 2.0).round(), 2.0);
        }
    }
}

fn despawn_pillars(mut commands: Commands, pillars: Query<Entity, With<Pillar>>) {
    for id in pillars.iter() {
        commands.entity(id).despawn();
    }
}

// THE CLOUDS /////////////////////////////////////////////////////////////////

// Component
struct Cloud;

fn spawn_clouds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    quad: Res<PixelSpriteQuad>,
    mut rng: ResMut<Rng>,
) {
    let clouds_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("flappin-clouds.png"),
        Vec2::new(CLOUD_WIDTH, CLOUD_HEIGHT),
        4,
        1,
    ));

    let mut x = LEFT;
    while x < RIGHT {
        let y = BOTTOM + 40.0 + rng.rand_range(0..(HEIGHT - 80.0 - CLOUD_HEIGHT) as u32) as f32;
        commands
            .spawn()
            .insert(Cloud)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: clouds_atlas.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: TextureAtlasSprite {
                    index: rng.rand_range(0..4),
                    ..Default::default()
                },
                mesh: quad.clone().into(),
                ..Default::default()
            });
        x += CLOUD_WIDTH;
    }
}

fn animate_clouds(
    mut rng: ResMut<Rng>,
    state: Res<State<GameState>>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Cloud>>,
) {
    if *state.current() == GameState::GameOver {
        return;
    }
    for (mut transform, mut sprite) in query.iter_mut() {
        *transform = transform.mul_transform(Transform::from_xyz(-1.0, 0.0, 0.0));
        if transform.translation.x + CLOUD_WIDTH < LEFT {
            let y = BOTTOM + 40.0 + rng.rand_range(0..(HEIGHT - 80.0 - CLOUD_HEIGHT) as u32) as f32;
            *transform = Transform::from_xyz(RIGHT, y, 0.0);
            sprite.index = rng.rand_range(0..4);
            sprite.flip_x = rng.rand_range(0..2) > 0;
        }
    }
}

// UTILITIES //////////////////////////////////////////////////////////////////

fn circle_box_collide(
    cx: f32,
    cy: f32,
    radius: f32,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
) -> bool {
    let closest_x = clamp(cx, left, right);
    let closest_y = clamp(cy, bottom, top);
    let distance_squared =
        (cx - closest_x) * (cx - closest_x) + (cy - closest_y) * (cy - closest_y);
    distance_squared < radius * radius
}

fn clamp(v: f32, lower: f32, upper: f32) -> f32 {
    lower.max(upper.min(v))
}

// RNG ////////////////////////////////////////////////////////////////////////

struct Rng {
    mz: u32,
    mw: u32,
}

impl Rng {
    fn rand(&mut self) -> u32 {
        self.mz = 36969 * (self.mz & 65535) + (self.mz >> 16);
        self.mw = 18000 * (self.mw & 65535) + (self.mw >> 16);
        u32::wrapping_add(self.mz << 16, self.mw)
    }

    fn rand_range(&mut self, range: std::ops::Range<u32>) -> u32 {
        let count = range.end - range.start;
        self.rand() % count + range.start
    }
}
