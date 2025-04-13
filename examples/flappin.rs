use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

// GAME CONSTANTS /////////////////////////////////////////////////////////////

const WIDTH: f32 = 240.0;
const HEIGHT: f32 = 240.0;
const LEFT: f32 = -WIDTH / 2.0;
const RIGHT: f32 = LEFT + WIDTH;
const BOTTOM: f32 = -HEIGHT / 2.0;
const _TOP: f32 = BOTTOM + HEIGHT;

const CLOUD_WIDTH: f32 = 66.0;
const CLOUD_HEIGHT: f32 = 20.0;

const PILLAR_WIDTH: f32 = 21.0;
const PILLAR_HEIGHT: f32 = 482.0;
const PILLAR_SPACING: f32 = 80.0;
const PILLAR_GAP: f32 = 70.0;
const PILLAR_RANGE: f32 = 105.0;

const BIRD_X: f32 = -80.0;
const BIRD_DX: f32 = 15.0;
const BIRD_DY: f32 = 11.0;
const BIRD_RADIUS: f32 = 6.0;

const FALLING_JERK: f32 = -2300.0;
const FLAP_VELOCITY: f32 = 100.0;
const FLAP_ACCELERATION: f32 = 90.0;

// SETUP //////////////////////////////////////////////////////////////////////

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
    #[default]
    StartScreen,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappin'".to_string(),
                    // resolution: bevy::window::WindowResolution::default()
                    //     .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),))
        .add_plugins(PixelCameraPlugin)
        .insert_state(GameState::default())
        .insert_resource(Rng { mz: 0, mw: 0 })
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(FlapTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .insert_resource(Action {
            just_pressed: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Startup, (spawn_bird, spawn_clouds).after(setup))
        .add_systems(Update, close_on_esc)
        .add_systems(Update, on_press)
        .add_systems(
            Update,
            (
                press_to_start,
                animate_flying_bird,
                animate_pillars,
                animate_clouds,
            )
                .run_if(in_state(GameState::StartScreen)),
        )
        .add_systems(OnEnter(GameState::Playing), spawn_pillars)
        .add_systems(
            Update,
            (
                flap,
                animate_flappin_bird,
                collision_detection,
                animate_pillars,
                animate_clouds,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(Update, press_to_start.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), despawn_pillars)
        .run();
}

#[derive(Resource)]
struct Textures {
    bird: Handle<Image>,
    bird_layout: Handle<TextureAtlasLayout>,
    pillars: Handle<Image>,
    clouds: Handle<Image>,
    clouds_layout: Handle<TextureAtlasLayout>,
}

fn setup(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<Rng>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    *rng = Rng {
        mz: time.elapsed().as_secs() as u32,
        mw: 678,
    };

    commands.insert_resource(Textures {
        bird: asset_server.load("flappin-bird.png"),
        bird_layout: atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(28, 23),
            4,
            1,
            None,
            None,
        )),
        pillars: asset_server.load("flappin-pillars.png"),
        clouds: asset_server.load("flappin-clouds.png"),
        clouds_layout: atlas_layouts.add(TextureAtlasLayout::from_grid(
            Vec2::new(CLOUD_WIDTH, CLOUD_HEIGHT).as_uvec2(),
            4,
            1,
            None,
            None,
        )),
    });

    commands.spawn((
        Camera2d,
        PixelZoom::FitSize {
            width: WIDTH as i32,
            height: HEIGHT as i32,
        },
        PixelViewport,
    ));
    // Deprecated:
    // commands.spawn(bevy_pixel_camera::PixelCameraBundle::from_resolution(
    //     WIDTH as i32,
    //     HEIGHT as i32,
    //     true,
    // ));
}

// INPUT MAPPING //////////////////////////////////////////////////////////////

#[derive(Resource)]
struct Action {
    just_pressed: bool,
}

fn on_press(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    // gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    touches: Res<Touches>,

    mut action: ResMut<Action>,
) {
    if keyboard.get_just_pressed().next().is_some()
        || mouse_buttons.get_just_pressed().next().is_some()
        // || gamepad_buttons.get_just_pressed().next().is_some()
        || touches.iter_just_pressed().next().is_some()
    {
        action.just_pressed = true;
    }
}

// START SCREEN ///////////////////////////////////////////////////////////////

fn press_to_start(
    mut action: ResMut<Action>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<FlapTimer>,
    mut birds: Query<(&mut Transform, &mut BirdPhysics), With<Bird>>,
) {
    timer.tick(time.delta());
    if !timer.finished() {
        action.just_pressed = false;
        return;
    }
    if action.just_pressed {
        action.just_pressed = false;
        for (mut transform, mut physics) in birds.iter_mut() {
            *transform = Transform::from_xyz(BIRD_X, 0.0, 1.0);
            physics.velocity = FLAP_VELOCITY;
            physics.acceleration = FLAP_ACCELERATION;
        }
        next_state.set(GameState::Playing);
    }
}

// THE BIRD ///////////////////////////////////////////////////////////////////

// Component
#[derive(Component)]
struct Bird;

// Component
#[derive(Component)]
struct BirdPhysics {
    velocity: f32,
    acceleration: f32,
}

#[derive(Component)]
struct BirdTimer(Timer);

fn mk_sprite(
    pos: Vec3,
    anchor: Anchor,
    index: usize,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
) -> (Sprite, Transform) {
    (
        Sprite {
            image,
            anchor,
            texture_atlas: Some(TextureAtlas { layout, index }),

            ..Default::default()
        },
        Transform::from_translation(pos),
    )
}

fn spawn_bird(mut commands: Commands, textures: Res<Textures>) {
    commands.spawn((
        Bird,
        BirdPhysics {
            velocity: 100.0,
            acceleration: 0.0,
        },
        mk_sprite(
            vec3(BIRD_X, 0.0, 1.0),
            Anchor::BottomLeft,
            0,
            textures.bird.clone(),
            textures.bird_layout.clone(),
        ),
        BirdTimer(Timer::from_seconds(0.150, TimerMode::Repeating)),
    ));
}

fn animate_flying_bird(
    time: Res<Time>,
    mut query: Query<(&mut BirdTimer, &mut Sprite), With<Bird>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let atlas = sprite.texture_atlas.as_mut().unwrap();

            atlas.index = (atlas.index + 1) % 4;
        }
    }
}

fn animate_flappin_bird(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BirdPhysics, &mut Sprite), With<Bird>>,
) {
    for (mut transform, mut physics, mut sprite) in query.iter_mut() {
        let dt = time.delta().as_secs_f32();
        let y = transform.translation.y + physics.velocity * dt;
        *transform = Transform::from_xyz(BIRD_X, y, 1.0);
        physics.velocity += physics.acceleration * dt;
        physics.acceleration += FALLING_JERK * dt;

        let atlas = sprite.texture_atlas.as_mut().unwrap();

        atlas.index = if physics.acceleration < -1200.0 {
            2
        } else if physics.acceleration > -300.0 {
            0
        } else {
            1
        };
    }
}

fn flap(mut action: ResMut<Action>, mut birds: Query<&mut BirdPhysics, With<Bird>>) {
    if action.just_pressed {
        action.just_pressed = false;
        for mut physics in birds.iter_mut() {
            physics.velocity = FLAP_VELOCITY;
            physics.acceleration = FLAP_ACCELERATION;
        }
    }
}

fn game_over(mut timer: ResMut<FlapTimer>, mut birds: Query<&mut Sprite, With<Bird>>) {
    timer.reset();

    for mut sprite in birds.iter_mut() {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = 3;
        }
    }
}

fn collision_detection(
    mut next_state: ResMut<NextState<GameState>>,
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
            next_state.set(GameState::GameOver);
        }
    }
}

// THE PILLARS ////////////////////////////////////////////////////////////////

// Component
#[derive(Component)]
struct Pillar;

fn spawn_pillars(mut commands: Commands, textures: Res<Textures>, mut rng: ResMut<Rng>) {
    let mut x = RIGHT;
    while x < RIGHT + WIDTH + PILLAR_SPACING {
        let y = (rng.rand_range(0..PILLAR_RANGE as u32) as f32 - PILLAR_RANGE / 2.0).round();
        commands.spawn((
            Pillar,
            Sprite {
                anchor: Anchor::BottomLeft,
                image: textures.pillars.clone(),
                ..Default::default()
            },
            Transform::from_xyz(x, (y - PILLAR_HEIGHT / 2.0).round(), 2.0),
        ));
        x += PILLAR_SPACING;
    }
}

fn animate_pillars(
    time: Res<Time>,
    mut rng: ResMut<Rng>,
    mut query: Query<&mut Transform, With<Pillar>>,
) {
    let dt = time.delta().as_secs_f32();
    for mut transform in query.iter_mut() {
        *transform = transform.mul_transform(Transform::from_xyz(-60.0 * dt, 0.0, 0.0));
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
#[derive(Component)]
struct Cloud;

fn spawn_clouds(mut commands: Commands, textures: Res<Textures>, mut rng: ResMut<Rng>) {
    let img = &textures.clouds;
    let layout = &textures.clouds_layout;
    let anchor = Anchor::CenterLeft;

    let mut x = LEFT;

    while x < RIGHT {
        let y = BOTTOM + 40.0 + rng.rand_range(0..(HEIGHT - 80.0 - CLOUD_HEIGHT) as u32) as f32;
        let index = rng.rand_range(0..4) as usize;
        let pos = vec3(x, y, 0.0);
        let spr = mk_sprite(pos, anchor, index, img.clone(), layout.clone());
        commands.spawn((Cloud, spr));
        x += CLOUD_WIDTH;
    }
}

fn animate_clouds(
    time: Res<Time>,
    mut rng: ResMut<Rng>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Cloud>>,
) {
    let dt = time.delta().as_secs_f32();

    for (mut transform, mut sprite) in query.iter_mut() {
        *transform = transform.mul_transform(Transform::from_xyz(-30.0 * dt, 0.0, 0.0));
        if transform.translation.x + CLOUD_WIDTH < LEFT {
            // Get atlas so we can set the index
            // Using unwrap as we expect all cloud sprites are atlas sprites
            let atlas = sprite.texture_atlas.as_mut().unwrap();

            let y = BOTTOM + 40.0 + rng.rand_range(0..(HEIGHT - 80.0 - CLOUD_HEIGHT) as u32) as f32;
            *transform = Transform::from_xyz(RIGHT, y, 0.0);
            atlas.index = rng.rand_range(0..4) as usize;
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

#[derive(Resource)]
struct FlapTimer(Timer);

impl std::ops::Deref for FlapTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FlapTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
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

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if focus.focused {
            if input.just_pressed(KeyCode::Escape) {
                commands.entity(window).despawn();
            }
        }
    }
}
