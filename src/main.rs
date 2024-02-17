//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::{
    prelude::*
    //, transform
};

mod globals;
mod construct;
use construct::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy-test-01".into(),
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
        ) 
        .add_systems(Startup, setup)
        .add_systems(Update, animate_builders)
        .add_systems(Update, character_movement)
        .run();
}

fn animate_builders(
    time: Res<Time>,
    mut query: Query<(&mut BuilderAnimator, &mut TextureAtlasSprite)>,
) {
    for (mut builder, mut sprite) in &mut query {
        match builder.state {

            BuildState::Undefined => {
                sprite.index = builder.progress.first;
                builder.state = BuildState::NotStarted;
                println!("Builder: {:?}", builder.state);
            }

            BuildState::NotStarted => {
                continue;
            },

            BuildState::Start => {
                println!("Builder: {:?}", builder.state);
                sprite.index = builder.progress.first;
                builder.progress_frame_time.reset();
                builder.state = BuildState::InProgress;
            },

            BuildState::InProgress => {
                builder.progress_frame_time.tick(time.delta());
                if builder.progress_frame_time.just_finished() {
                    sprite.index = sprite.index + 1;
                    println!("Builder: {:?} {}/{}",
                        builder.state,
                        sprite.index,
                        builder.progress.last
                    );
                    builder.progress_frame_time.reset();
                    if sprite.index == builder.progress.last {
                        builder.state = BuildState::Finish;
                        println!("Builder: {:?}", builder.state);
                        builder.showcase_time.reset();
                    };
                };
            },

            BuildState::Finish => {
                sprite.index = builder.progress.last;
                builder.showcase_time.tick(time.delta());
                if builder.showcase_time.just_finished() && (builder.build_type == BuilderType::AutoRepeat) {
                    builder.state = BuildState::Start;
                };
            },
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("build_chair.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 64.0), 8, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 7 };

    commands.spawn(
        BuilderAnimator {
            state: BuildState::Start,
            build_type: BuilderType::AutoRepeat,
            progress: AnimationIndices { first: 0, last: 7 },
            progress_frame_time: Timer::from_seconds(0.2, TimerMode::Once),
            showcase_time: Timer::from_seconds(1.0, TimerMode::Once)
    }).insert(
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
    },);

    
    commands.spawn(Camera2dBundle::default());
}

fn character_movement(
    mut characters: Query<(&mut Transform, &TextureAtlasSprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}