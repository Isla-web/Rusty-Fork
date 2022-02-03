use std::collections::HashMap;

use serde::Deserialize;

// sceneX.lua

#[derive(Deserialize, PartialEq, Debug)]
pub struct Vector<T> {
    #[serde(default)]
    pub x: T,
    #[serde(default)]
    pub y: T,
    #[serde(default)]
    pub z: T,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct BlockRect {
    pub min: Vector<f32>,
    pub max: Vector<f32>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct SceneConfig {
    pub born_rot: Vector<f32>,
    pub born_pos: Vector<f32>,
    pub begin_pos: Vector<f32>,
    pub size: Vector<f32>,
    #[serde(default)]
    pub die_y: f32,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Scene {
    pub blocks: HashMap<u32,u32>,
    #[serde(default)]
    pub block_rects: HashMap<u32,BlockRect>,
    #[serde(default)]
    pub routes_config: HashMap<u32,String>,
    #[serde(default)]
    pub dummy_points: HashMap<u32,String>,
    pub scene_config: SceneConfig,
}

// sceneX_blockY.lua

#[derive(Deserialize, PartialEq, Debug)]
pub struct Block {
    pub groups: HashMap<u32,GroupInfo>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GroupInfo {
    pub is_replaceable: Option<ComplicatedBool>,
    #[serde(default)]
    pub dynamic_load: bool,
    pub id: u32,
    pub area: Option<u32>,
    pub pos: Vector<f32>,
    pub business: Option<Business>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ComplicatedBool {
    pub version: u32,
    pub value: bool,
    pub new_bin_only: bool,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Business {
    pub r#type: u32,
}

// sceneX_groupZ.lua

#[derive(Deserialize, PartialEq, Debug)]
pub struct Group {
    pub init_config: Option<GroupInitConfig>,
    #[serde(default)]
    pub suites: HashMap<u32,Suite>,

    #[serde(default)]
    pub npcs: HashMap<u32,Npc>,
    #[serde(default)]
    pub variables: HashMap<u32,Variable>,
    #[serde(default)]
    pub triggers: HashMap<u32,u32>,
    #[serde(default)]
    pub regions: HashMap<u32,u32>,
    #[serde(default)]
    pub gadgets: HashMap<u32,Gadget>,
    #[serde(default)]
    pub monsters: HashMap<u32,Monster>,

    // MovePlatform - Function???
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Suite {
    pub rand_weight: u32,

    #[serde(default)]
    pub npcs: HashMap<u32,u32>,
    // Variables?
    #[serde(default)]
    pub triggers: HashMap<u32,u32>,
    #[serde(default)]
    pub regions: HashMap<u32,u32>,
    #[serde(default)]
    pub gadgets: HashMap<u32,u32>,
    #[serde(default)]
    pub monsters: HashMap<u32,u32>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GroupInitConfig {
    pub end_suite: Option<u32>,
    //#[serde(default)]
    pub rand_suite: bool,
    pub suite: u32,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Variable {
    pub name: String,
    pub value: u32,
    pub no_refresh: bool,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Monster {
    pub rot: Vector<f32>,
    pub pos: Vector<f32>,
    pub config_id: u32,
    pub level: u32,
    pub monster_id: u32,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Npc {
    pub rot: Vector<f32>,
    pub pos: Vector<f32>,
    pub config_id: u32,
    pub npc_id: u32,

    pub area_id: Option<u32>,
    pub room: Option<u32>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Gadget {
    pub rot: Vector<f32>,
    pub pos: Vector<f32>,
    pub config_id: u32,
    pub level: u32,
    pub gadget_id: u32,

    #[serde(default)]
    pub drop_count: u32,
    pub explore: Option<ExploreInfo>,
    #[serde(default)]
    pub isOneoff: bool,
    pub area_id: Option<u32>,
    #[serde(default)]
    pub persistent: bool,
    pub chest_drop_id: Option<u32>,
    #[serde(default)]
    pub start_route: bool,
    #[serde(default)]
    pub is_use_point_array: bool,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ExploreInfo {
    pub exp: u32,
    pub name: String,
}