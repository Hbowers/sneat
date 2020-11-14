use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::World,
    prelude::WorldExt,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheetFormat, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    Sneatling,
    GroundBlock,
}

#[derive(Default)]
pub struct SpriteSheetTable {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}

impl SpriteSheetTable {
    pub fn insert(&mut self, asset_type: AssetType, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(asset_type, sprite_sheet_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

pub fn load_sprite_sheet_by_asset(world: &mut World, asset: AssetType) -> Handle<SpriteSheet> {
    let (path_to_sprite, path_to_sprite_config) = match asset {
        AssetType::Sneatling => (
            "sprites/Whale/1-Idle/1.png",
            "sprite_configs/sneatling_spritesheet.ron",
        ),
        AssetType::GroundBlock => (
            "sprites/environment/tileset.png",
            "sprite_configs/environment_spritesheet.ron",
        ),
    };

    load_sprite_sheet_by_path(world, path_to_sprite, path_to_sprite_config)
}

fn load_sprite_sheet_by_path(
    world: &mut World,
    path_to_sprite: &str,
    path_to_sprite_config: &str,
) -> Handle<SpriteSheet> {
    let texture_handler = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(path_to_sprite, ImageFormat::default(), (), &texture_storage)
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        path_to_sprite_config,
        SpriteSheetFormat(texture_handler),
        (),
        &sprite_sheet_store,
    )
}
