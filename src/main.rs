use bevy::prelude::*;
use bevy::math::Vec2;

use components::SpriteSize;
use pokemon::PokemonPlugin;

mod components;
mod pokemon;

// region:	--- Asset Constants
const POKEMON1_SPRITE: &str = "pokemon1.png";
const POKEMON2_SPRITE: &str = "pokemon2.png";
const POKEMON3_SPRITE: &str = "pokemon3.png";
const ARROW_SPRITE: &str = "arrow.png";
const POKEMON_SIZE: Vec2 = Vec2{ x: 144., y: 75.};
const SPRITE_SCALE: f32 = 0.5;
// endregion:	--- Asset Constants

// region:	--- Resource Constants
#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
	pokemon1: Handle<Image>,
	pokemon2: Handle<Image>,
	pokemon3: Handle<Image>,
	arrow: Handle<Image>,
}
// endregion:	--- Resource Constants

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::WHITE))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "PokeMaze".to_string(),
				width: 598.0,
				height: 676.0,
				..Default::default()
			},
			..Default::default()
		}))
		.add_plugin(PokemonPlugin)
		.add_startup_system(setup_system)
    .run();
}

fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut windows: ResMut<Windows>,
) {
		// camera
		commands.spawn(Camera2dBundle::default());

		let window = windows.get_primary_mut().unwrap();
		let (win_w, win_h) = (window.width(), window.height());
		let win_size = WinSize { w: win_w, h: win_h };
		commands.insert_resource(win_size);

		let game_textures = GameTextures {
			pokemon1: asset_server.load(POKEMON1_SPRITE),
			pokemon2: asset_server.load(POKEMON2_SPRITE),
			pokemon3: asset_server.load(POKEMON3_SPRITE),
			arrow: asset_server.load(ARROW_SPRITE),
		};
		commands.insert_resource(game_textures);
}