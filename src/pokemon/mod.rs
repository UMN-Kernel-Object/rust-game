use crate::components::{ Pokemon, SpriteSize, Movable };

use crate::{
  GameTextures, POKEMON_SIZE, SPRITE_SCALE
};

use bevy::{prelude::*, transform};

pub struct PokemonPlugin;

impl Plugin for PokemonPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(pokemon_partner_system)
      .add_system(move_arrow_system)
      .add_system(select_arrow_system);
  }
}

fn pokemon_partner_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
) {
  commands
    .spawn(SpriteBundle {
      texture: game_textures.pokemon1.clone(),
      transform: Transform {
        translation: Vec3::new(
          0.,
          0.,
          10.,
        ),
        scale: Vec3::new(SPRITE_SCALE * 2., SPRITE_SCALE * 2., 1.),
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(Pokemon)
    .insert(SpriteSize(POKEMON_SIZE));
  commands
    .spawn(SpriteBundle {
      texture: game_textures.pokemon2.clone(),
      transform: Transform {
        translation: Vec3::new(
          -100.,
          0.,
          10.,
        ),
        scale: Vec3::new(SPRITE_SCALE * 2., SPRITE_SCALE * 2., 1.),
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(Pokemon)
    .insert(SpriteSize(POKEMON_SIZE));
  commands
    .spawn(SpriteBundle {
      texture: game_textures.pokemon3.clone(),
      transform: Transform {
        translation: Vec3::new(
          100.,
          0.,
          10.,
        ),
        scale: Vec3::new(SPRITE_SCALE * 2., SPRITE_SCALE * 2., 1.),
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(Pokemon)
    .insert(SpriteSize(POKEMON_SIZE));
}
fn select_arrow_system(
  mut commands: Commands,
  game_textures: Res<GameTextures>,
) {
  let initial_x: f32 = -100.;
  commands
  .spawn(SpriteBundle {
    texture: game_textures.arrow.clone(),
    transform: Transform {
      translation: Vec3::new(initial_x, -25., 0.), 
      scale: Vec3::new(SPRITE_SCALE / 20., SPRITE_SCALE / 20., 1.),
      rotation: Quat::from_rotation_z(std::f32::consts::PI),
      ..Default::default()
    },
    ..Default::default()
  })
    .insert(Movable { is_movable: true });
}

fn move_arrow_system(
  kb: Res<Input<KeyCode>>,
  mut query: Query<&mut Transform, With<Movable>>,
) {
  for(mut transform) in query.iter_mut() {
    let mut x: f32 = transform.translation.x;
    if kb.just_released(KeyCode::Left) {
      if x != 0. {
          x -= 100.
      }      
    }
    else if kb.just_released(KeyCode::Right) {
      if x != 200. {
          x += 100.
      }      
    }
    transform.translation.x = x;
  }
}