// Database Manager
use std::collections::HashMap;

use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult, Database};
use sea_orm::entity::prelude::*;
use crate::JsonManager;
use crate::server::AuthManager;
use crate::utils::IdManager;

pub use super::player_info::Model as PlayerInfo;
use super::player_info::Entity as PlayerInfoEntity;

pub use super::avatar_info::Model as AvatarInfo;
use super::avatar_info::Entity as AvatarInfoEntity;

pub use super::scene_info::Model as SceneInfo;
use super::scene_info::Entity as SceneInfoEntity;

pub use super::team_info::Model as TeamInfo;
use super::team_info::Entity as TeamInfoEntity;

pub use super::avatar_team_info::Model as AvatarTeamInfo;
use super::avatar_team_info::Entity as AvatarTeamInfoEntity;

pub use super::team_selection_info::Model as TeamSelectionInfo;
use super::team_selection_info::Entity as TeamSelectionInfoEntity;

pub use super::player_prop::Model as PlayerProp;
use super::player_prop::Entity as PlayerPropEntity;

pub use super::avatar_prop::Model as AvatarProp;
use super::avatar_prop::Entity as AvatarPropEntity;

pub use super::avatar_fight_prop::Model as AvatarFightProp;
use super::avatar_fight_prop::Entity as AvatarFightPropEntity;

pub use super::open_state::Model as OpenState;
use super::open_state::Entity as OpenStateEntity;

/* Inventory */
pub use super::material_info::Model as MaterialInfo;
use super::material_info::Entity as MaterialInfoEntity;

pub use super::reliquary_info::Model as ReliquaryInfo;
use super::reliquary_info::Entity as ReliquaryInfoEntity;

pub use super::equip_info::Model as EquipInfo;
use super::equip_info::Entity as EquipInfoEntity;

pub use super::item_info::Model as ItemInfo;
use super::item_info::Entity as ItemInfoEntity;

pub use super::weapon_affix_info::Model as WeaponAffixInfo;
use super::weapon_affix_info::Entity as WeaponAffixInfoEntity;

pub use super::reliquary_prop::Model as ReliquaryProp;
use super::reliquary_prop::Entity as ReliquaryPropEntity;

pub use super::furniture_info::Model as FurnitureInfo;
use super::furniture_info::Entity as FurnitureInfoEntity;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

trait Block {
    fn wait(self) -> <Self as futures::Future>::Output
        where Self: Sized, Self: futures::Future
    {
        futures::executor::block_on(self)
    }
}

impl<F,T> Block for F
    where F: futures::Future<Output = T>
{}

pub struct DatabaseManager {
    db: DbConn,
}

impl DatabaseManager {
    pub fn new(conn_string: &str) -> Self {
        return DatabaseManager {
            db: Database::connect(conn_string).wait().unwrap(),
        };
    }

    pub fn get_player_info(&self, uid: u32) -> Option<PlayerInfo> {
        match PlayerInfoEntity::find_by_id(uid).one(&self.db).wait() {
            Err(_) => { println!("DB ERROR!"); None },
            Ok(p_info) => p_info,
        }
    }
/*
    pub fn _get_player_info(&self, uid: u32) -> Option<PlayerInfo> {
        Some(PlayerInfo {
            uid: uid,
            nick_name: "Fapper".into(),
            signature: "Hello world!".into(),
            birthday: 0,
            namecard_id: 210051,
            finish_achievement_num: 42,
            tower_floor_index: 1,
            tower_level_index: 1,
            avatar_id: 10000007,
        })
    }*/
/*
    pub fn _get_player_props(&self, uid: u32) -> Option<HashMap<u32, i64>> {
        Some(collection! {
            //proto::PropType::PropIsSpringAutoUse as u32 => 1,
            //proto::PropType::PropIsFlyable as u32 => 1,
            //proto::PropType::PropIsTransferable as u32 => 1,
            //proto::PropType::PropPlayerLevel as u32 => 56,
            //proto::PropType::PropPlayerExp as u32 => 1337,
            //proto::PropType::PropPlayerHcoin as u32 => 9001,
            //proto::PropType::PropPlayerScoin as u32 => 9002,
            //proto::PropType::PropPlayerWorldLevel as u32 => 8,
            //proto::PropType::PropPlayerResin as u32 => 159,
            //proto::PropType::PropPlayerMcoin as u32 => 9003,
            //proto::PropType::PropMaxStamina as u32 => 12000,
            //proto::PropType::PropCurPersistStamina as u32 => 12000,
        })
    }*/

    pub fn get_player_props(&self, uid: u32) -> Option<HashMap<u32, i64>> {
        let props = match PlayerPropEntity::find_by_id(uid).all(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(p_info) => p_info,
        };

        let props = props
            .into_iter()
            .map(|p| (p.prop_id, p.prop_value))
            .collect();

        return Some(props);
    }
/*
    pub fn _get_avatar_props(&self, guid: u64) -> Option<HashMap<u32, i64>> {
        let map = collection! {
            //proto::PropType::PropExp as u32 => 0,
            //proto::PropType::PropLevel as u32 => 80,
            //proto::PropType::PropBreakLevel as u32 => 5,
            //proto::PropType::PropSatiationVal as u32 => 0,
            //proto::PropType::PropSatiationPenaltyTime as u32 => 0,
        };

        return Some(map);
    }*/

    pub fn get_avatar_props(&self, guid: i64) -> Option<HashMap<u32, i64>> {
        let props = match AvatarPropEntity::find_by_id(guid).all(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(p_info) => p_info,
        };

        let props = props
            .into_iter()
            .map(|p| (p.prop_id, p.prop_value))
            .collect();

        return Some(props);
    }

    pub fn get_avatar_equip(&self, guid: i64) -> Option<Vec<i64>> {
        let equip = vec![Self::SPOOFED_WEAPON_GUID];

        return Some(equip);
    }

    pub fn get_skill_levels(&self, guid: i64) -> Option<HashMap<u32,u32>> {
        let map = collection! {
            10068 => 3,
            100553 => 3,
            10067 => 3,
        };

        return Some(map);
    }

    pub fn get_avatar_fight_props(&self, guid: i64) -> Option<HashMap<u32, f32>> {
        /*
        let map = collection! {
            proto::FightPropType::FightPropBaseHp as u32 => 9000.0,
            proto::FightPropType::FightPropHp as u32 => 3000.0,
            proto::FightPropType::FightPropHpPercent as u32 => 0.0746000,

            proto::FightPropType::FightPropBaseAttack as u32 => 600.0,
            proto::FightPropType::FightPropAttack as u32 => 50.0,
            proto::FightPropType::FightPropAttackPercent as u32 => 0.40,

            proto::FightPropType::FightPropBaseDefense as u32 => 600.0,
            proto::FightPropType::FightPropDefense as u32 => 40.0,
            proto::FightPropType::FightPropDefensePercent as u32 => 0.04,

            proto::FightPropType::FightPropCritical as u32 => 0.99,
            proto::FightPropType::FightPropAntiCritical as u32 => 0.00000,
            proto::FightPropType::FightPropCriticalHurt as u32 => 0.99,
            proto::FightPropType::FightPropChargeEfficiency as u32 => 1.337,

            proto::FightPropType::FightPropHealAdd as u32 => 0.00000,
            proto::FightPropType::FightPropHealedAdd as u32 => 0.00000,
            proto::FightPropType::FightPropElementMastery as u32 => 42.0,

            proto::FightPropType::FightPropPhysicalSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropPhysicalAddHurt as u32 => 0.271828,

            proto::FightPropType::FightPropFireAddHurt as u32 => 0.00000,
            proto::FightPropType::FightPropElecAddHurt as u32 => 0.00000,
            proto::FightPropType::FightPropWaterAddHurt as u32 => 0.00000,
            proto::FightPropType::FightPropGrassAddHurt as u32 => 0.00000,
            proto::FightPropType::FightPropWindAddHurt as u32 => 0.00000,
            proto::FightPropType::FightPropRockAddHurt as u32 => 0.00000,
            proto::FightPropType::FightPropIceAddHurt as u32 => 0.00000,

            proto::FightPropType::FightPropFireSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropElecSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropWaterSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropGrassSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropWindSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropRockSubHurt as u32 => 0.00000,
            proto::FightPropType::FightPropIceSubHurt as u32 => 0.00000,

            proto::FightPropType::FightPropMaxWindEnergy as u32 => 60.0000,

            proto::FightPropType::FightPropCurWindEnergy as u32 => 60.0000,

            proto::FightPropType::FightPropCurHp as u32 => 10000.0,

            proto::FightPropType::FightPropMaxHp as u32 => 12000.0,
            proto::FightPropType::FightPropCurAttack as u32 => 900.0,
            proto::FightPropType::FightPropCurDefense as u32 => 700.0,
            proto::FightPropType::FightPropCurSpeed as u32 => 10.00000,
        };

        return Some(map);

         */
        let props = match AvatarFightPropEntity::find_by_id(guid).all(&self.db).wait() {
            Err(e) => { panic!("DB ERROR {}: {}!", guid, e) },
            Ok(props) => props,
        };

        let props = props
            .into_iter()
            .map(|p| (p.prop_id, p.value))
            .collect();

        return Some(props);
    }

    pub fn get_open_state(&self, uid: u32) -> Option<HashMap<u32, u32>> {
        /*
        Some(collection! {
            proto::OpenStateType::OpenStatePaimon as u32 => 1,

            proto::OpenStateType::OpenStatePlayerLvupGuide as u32 => 1,

            proto::OpenStateType::OpenStateGacha as u32 => 1,
            proto::OpenStateType::OpenStateGuideGacha as u32 => 1,

            proto::OpenStateType::OpenStateGuideTeam as u32 => 1,

            proto::OpenStateType::OpenStateGuideBag as u32 => 1,

            proto::OpenStateType::OpenStateLimitRegionFreshmeat as u32 => 1,
            proto::OpenStateType::OpenStateLimitRegionGlobal as u32 => 1,
            proto::OpenStateType::OpenStateMultiplayer as u32 => 0,

            proto::OpenStateType::OpenStateAvatarFashion as u32 => 1,

            proto::OpenStateType::OpenStateGuideAppearance as u32 => 1,

            proto::OpenStateType::OpenStateShopTypeMall as u32 => 1, // 900
            proto::OpenStateType::OpenStateShopTypeRecommanded as u32 => 1, // 901
            proto::OpenStateType::OpenStateShopTypeGenesiscrystal as u32 => 1, // 902
            proto::OpenStateType::OpenStateShopTypeGiftpackage as u32 => 1, // 903

            proto::OpenStateType::OpenAdventureManual as u32 => 1, // 1100
            proto::OpenStateType::OpenAdventureManualMonster as u32 => 1, // 1103
            proto::OpenStateType::OpenAdventureManualBossDungeon as u32 => 1, // 1104

            proto::OpenStateType::OpenStateMengdeInfusedcrystal as u32 => 1,
            proto::OpenStateType::OpenStateLiyueInfusedcrystal as u32 => 1,
            proto::OpenStateType::OpenStateInazumaMainquestFinished as u32 => 1,
        })

         */
        let states = match OpenStateEntity::find_by_id(uid).all(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(states) => states,
        };

        let states = states
            .into_iter()
            .map(|s| (s.state_id, s.value))
            .collect();

        return Some(states);
    }

    pub fn get_inventory(&self, uid: u32) -> Option<Vec<proto::Item>> {

        let mut weapon = proto::Weapon::default();
        weapon.level = 70;
        weapon.promote_level = 4;
        weapon.affix_map = collection!{111406 => 0};

        let mut equip = proto::Equip::default();
        equip.is_locked = true;
        equip.detail = Some(proto::equip::Detail::Weapon(weapon));

        let mut item = proto::Item::default();
        item.item_id = 11406;
        item.guid = Self::SPOOFED_WEAPON_GUID as u64; // FIXME
        item.detail = Some(proto::item::Detail::Equip(equip));

        return Some(vec![item]);



        /*
         Inventory item can be of three types: material, equip and furniture
         Equip is further divided into relic and weapon
         Sp we need to get:
         1) Materials
         2) Furniture
         3) Relics (+their properties)
         4) Weapons (+their affices)
         */
        /*
        let items = match ItemInfoEntity::find_by_id(uid).all(&self.db).wait() {
            Err(e) => { panic!("DB ERROR: {}!", e) },
            Ok(items) => items,
        };

        let materials: Vec<(ItemInfo, MaterialInfo)> = self.find_related_to_items(&items, MaterialInfoEntity);

        let furniture: Vec<(ItemInfo, FurnitureInfo)> = self.find_related_to_items(&items, FurnitureInfoEntity);

        let equip: Vec<(ItemInfo, EquipInfo)> = self.find_related_to_items(&items, EquipInfoEntity);

        return None;*/
    }

    fn find_related_to_items<T: sea_orm::EntityTrait>(&self, items: &Vec<ItemInfo>, entity_type: T) -> Vec<(ItemInfo, T::Model)>
        where
            ItemInfoEntity: sea_orm::Related<T>
    {
        return items.into_iter()
            .map(|item| {
                let ret = match item.find_related(entity_type).one(&self.db).wait() {
                    Err(e) => { panic!("DB ERROR: {}!", e) },
                    Ok(data) => data,
                };

                match ret {
                    None => None,
                    Some(data) => Some( (item.clone(), data) ),
                }
            })
            .filter(|x| !x.is_none())
            .map(|x| x.unwrap())
            .collect();
    }

    pub fn get_avatars(&self, uid: u32) -> Option<Vec<AvatarInfo>> {
        let avatars = match AvatarInfoEntity::find_by_id(uid).all(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(avatars) => avatars,
        };

        return Some(avatars);
    }
/*
    pub fn _get_avatars(&self, uid: u32) -> Option<Vec<AvatarInfo>> {
        let ai = AvatarInfo {
            uid: uid,
            character_id: 7,
            avatar_type: 1,
            guid: Self::SPOOFED_AVATAR_GUID,
            born_time: 1633790000,
        };

        return Some(vec![ai]);


    }*/

    pub fn get_avatar(&self, guid: i64) -> Option<AvatarInfo> {
        let avatar = match AvatarInfoEntity::find().filter(super::avatar_info::Column::Guid.eq(guid)).one(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(avatar) => avatar,
        };

        return avatar;
    }
/*
    pub fn _get_avatar(&self, guid: u64) -> Option<AvatarInfo> {
        let ai = AvatarInfo {
            uid: AuthManager::SPOOFED_PLAYER_UID, // TODO!
            character_id: 7,
            avatar_type: 1,
            guid: Self::SPOOFED_AVATAR_GUID,
            born_time: 1633790000,
        };

        return Some(ai);
    }*/
/*
    pub fn _get_player_scene_info(&self, uid: u32) -> Option<SceneInfo> {
        let si = SceneInfo {
            uid: uid,
            scene_id: Self::SPOOFED_SCENE_ID,
            scene_token: Self::SPOOFED_SCENE_TOKEN,
            pos_x: -3400.0,
            pos_y: 233.0,
            pos_z: -3427.6,
        };

        return Some(si);
    }
*/
    pub fn get_player_scene_info(&self, uid: u32) -> Option<SceneInfo> {
        let scene_info = match SceneInfoEntity::find_by_id(uid).one(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(info) => info,
        };

        return scene_info;
    }

    pub fn get_player_teams(&self, uid: u32) -> Option<Vec<TeamInfo>> {
        /*let t1 = TeamInfo {
            uid: uid.clone(),
            id: 1,
            name: "Team 1".to_string(),
        };

        let t2 = TeamInfo {
            uid: uid.clone(),
            id: 2,
            name: "Team 2".to_string(),
        };

        let t3 = TeamInfo {
            uid: uid.clone(),
            id: 3,
            name: "Team 3".to_string(),
        };

        let t4 = TeamInfo {
            uid: uid.clone(),
            id: 4,
            name: "Team 4".to_string(),
        };

        return Some(vec![t1, t2, t3, t4]);
        */
        let teams = match TeamInfoEntity::find_by_id(uid).all(&self.db).wait() {
            Err(_) => panic!("Failed to retrieve teams for user {}!", uid),
            Ok(teams) => teams,
        };

        return Some(teams);
    }

    pub fn get_player_teams_avatars(&self, uid: u32) -> Option<Vec<AvatarTeamInfo>> {
        /*
        let a1 = AvatarTeamInfo {
            uid: uid.clone(),
            team_id: 1,
            guid: Self::SPOOFED_AVATAR_GUID,
        };

        let a2 = AvatarTeamInfo {
            uid: uid.clone(),
            team_id: 2,
            guid: Self::SPOOFED_AVATAR_GUID,
        };

        let a3 = AvatarTeamInfo {
            uid: uid.clone(),
            team_id: 3,
            guid: Self::SPOOFED_AVATAR_GUID,
        };

        let a4 = AvatarTeamInfo {
            uid: uid.clone(),
            team_id: 4,
            guid: Self::SPOOFED_AVATAR_GUID,
        };

        return Some(vec![a1, a2, a3, a4]);
         */
        let teams = match AvatarTeamInfoEntity::find_by_id(uid).all(&self.db).wait() {
            Err(_) => panic!("Failed to retrieve avatar teams for user {}!", uid),
            Ok(teams) => teams,
        };

        return Some(teams);
    }

    pub fn get_player_team_selection(&self, uid: u32) -> Option<TeamSelectionInfo> {
        /*
        let tsi = TeamSelectionInfo {
            uid: uid.clone(),
            avatar: Self::SPOOFED_AVATAR_GUID,
            team: 1,
        };

        return Some(tsi);
         */
        let tsi = match TeamSelectionInfoEntity::find_by_id(uid).one(&self.db).wait() {
            Err(_) => { panic!("DB ERROR!") },
            Ok(info) => info,
        };

        return tsi;
    }

    const BASE_GUID: i64 = 0x2400000000000000;
    const SPOOFED_AVATAR_GUID: i64 = Self::BASE_GUID + 1;
    const SPOOFED_WEAPON_GUID: i64 = Self::BASE_GUID + 2;
    const SPOOFED_SCENE_ID: u32 = 3;
    const SPOOFED_SCENE_TOKEN: u32 = 0x1234;
}
