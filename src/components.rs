use bevy::prelude::Component;
use bevy::math::{Vec2, Vec3};

// region:  --- Common Components
#[derive(Component)]
pub struct Movable {
  pub is_movable: bool,
}

#[derive(Component)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
// endregion: --- Common Components

// region:  --- Pokemon Components
#[derive(Component)]
pub struct Pokemon;
// endregion: --- Pokemon Components

