use amethyst::{
    core::transform::Transform,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::components::{Animation, Barrel, Coverable, Covers, Floor, Shape, Sneatling, Velocity};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::entities::{barrel, camera, camera_focus, cover, floor, sneatling};
use crate::resources::assets;

pub struct Sneat;
pub struct Paused;

#[derive(Serialize, Deserialize)]
pub enum EntityType {
    Sneatling,
    Barrel,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDetail {
    entity_type: EntityType,
    health: f32,
    x: f32,
    y: f32,
}
#[derive(Serialize, Deserialize)]
pub struct FloorDetail {
    y: f32,
    x_start: f32,
    x_end: f32,
}

#[derive(Serialize, Deserialize)]
pub struct CoverDetail {
    y: f32,
    x_start: f32,
    x_end: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    entities: Vec<EntityDetail>,
    floors: Vec<FloorDetail>,
    cover: Vec<CoverDetail>,
}

impl SimpleState for Sneat {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut app_root = env::current_exe().expect("Cannot get current path");
        app_root.pop();

        let level_path = app_root.join("levels").join("level_1.ron");
        println!("Loading level 1 from: {:?}", level_path);
        let s = fs::read_to_string(level_path).expect("Could not find file");
        let level: Level = ron::de::from_str(&s).expect("ITS THIS ERROR");

        let sneatling_sprite_sheet_handle =
            assets::load_sprite_sheet_by_asset(world, assets::AssetType::Sneatling);
        let environment_sprite_sheet_handle =
            assets::load_sprite_sheet_by_asset(world, assets::AssetType::EnvironmentBlock);
        let barrel_sprite_sheet_handle =
            assets::load_sprite_sheet_by_asset(world, assets::AssetType::Barrel);

        world.register::<Sneatling>();
        world.register::<Velocity>();
        world.register::<Barrel>();
        world.register::<Floor>();
        world.register::<Animation>();
        world.register::<Shape>();
        world.register::<Covers>();
        world.register::<Coverable>();

        let focus = camera_focus::initialise_camera_focus(
            world,
            ARENA_WIDTH * 0.5,
            ARENA_HEIGHT * 0.5,
            1.0,
        );
        camera::initialise_camera(world, focus, ARENA_WIDTH, ARENA_HEIGHT);
        for floor in level.floors {
            floor::initialise_flooring(
                world,
                floor.x_start,
                floor.x_end,
                floor.y,
                environment_sprite_sheet_handle.clone(),
            );
        }
        for cover in level.cover {
            cover::initialise_covering(
                world,
                cover.x_start,
                cover.x_end,
                cover.y,
                environment_sprite_sheet_handle.clone(),
            );
        }

        for ent in level.entities {
            match ent.entity_type {
                EntityType::Sneatling => {
                    sneatling::initialise_sneatling(
                        world,
                        sneatling_sprite_sheet_handle.clone(),
                        ent.health,
                    );
                }
                EntityType::Barrel => {
                    barrel::initialise_barrel(
                        world,
                        (ent.x, ent.y),
                        barrel_sprite_sheet_handle.clone(),
                    );
                }
            }
        }
    }
}
