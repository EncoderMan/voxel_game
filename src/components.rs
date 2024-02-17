use bevy::{
    prelude::*
    //, transform
};

#[derive(Component)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Component, Copy, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(PartialEq, Debug)]
pub enum BuildState{
    Undefined,
    NotStarted,
    Start,
    InProgress,
    Finish
}

#[derive(PartialEq, Debug)]
pub enum BuilderType {
    ManualOnce,
    ManualRepeat,
    AutoOnce,
    AutoRepeat
}

#[derive(Component)]
pub struct BuilderAnimator {
    //pub sprite_bundle: SpriteSheetBundle,
    pub state: BuildState,
    pub build_type: BuilderType,
    pub progress: AnimationIndices,
    pub progress_frame_time: Timer,
    pub showcase_time: Timer
}