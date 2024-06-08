use bevy::prelude::*;
use bevy::utils::HashMap;

pub const PLAYER_IMG_PATH: &str = "player.png";
pub const SQUID_IMG_PATH: &str = "squid.png";
pub const CRAB_IMG_PATH: &str = "crab.png";
pub const OCTPUS_IMG_PATH: &str = "octopus.png";
pub const SIMPLE_MISSILE_IMG_PATH: &str = "simple_missile.png";

#[derive(PartialEq, Eq, Hash)]
pub enum Textures {
    Player,
    Squid,
    Crab,
    Octopus,
    SimpleMissile,
}

#[derive(Resource)]
pub struct GameTextures {
    textures: HashMap<Textures, Handle<Image>>,
}

impl GameTextures {
    pub fn insert_texture(&mut self, key: Textures, value: Handle<Image>) {
        self.textures.insert(key, value);
    }

    pub fn get_texture(&self, key: Textures) -> Handle<Image> {
        self.textures.get(&key).unwrap().clone()
    }
}

pub struct GameTexturesBuilder;

impl GameTexturesBuilder {
    pub fn build(assets: Res<AssetServer>) -> GameTextures {
        let mut game_textures = GameTextures {
            textures: HashMap::new(),
        };
        game_textures.insert_texture(Textures::Player, assets.load(PLAYER_IMG_PATH));
        game_textures.insert_texture(Textures::Squid, assets.load(SQUID_IMG_PATH));
        game_textures.insert_texture(Textures::Crab, assets.load(CRAB_IMG_PATH));
        game_textures.insert_texture(Textures::Octopus, assets.load(OCTPUS_IMG_PATH));
        game_textures.insert_texture(
            Textures::SimpleMissile,
            assets.load(SIMPLE_MISSILE_IMG_PATH),
        );
        game_textures
    }
}
