use crate::{define_netvar, sdk::EntHandle};

use super::{ids::ItemDefinitionInex, HasNetvars, Weapon};


impl HasNetvars for Weapon {
    fn get_class_name() -> &'static str {
        "CTFWeaponBase"
    }
}
impl Weapon {
    define_netvar!(
        get_item_definition_index,
        [
            "baseclass",
            "baseclass",
            "m_AttributeManager",
            "m_Item",
            "m_iItemDefinitionIndex"
        ],
        ItemDefinitionInex
    );
    define_netvar!(
        get_next_primary_attack,
        [
            "baseclass",
            "LocalActiveWeaponData",
            "m_flNextPrimaryAttack"
        ],
        f32
    );
    define_netvar!(
        get_last_fire,
        ["LocalActiveTFWeaponData", "m_flLastFireTime"],
        f32
    );

    define_netvar!(get_clip1, ["baseclass", "LocalWeaponData", "m_iClip1"], i32);
    define_netvar!(
        get_owner,
        [
            "baseclass",
            "baseclass",
            "baseclass",
            "baseclass",
            "m_hOwnerEntity"
        ],
        EntHandle
    );
    define_netvar!(
        get_observed_crit_chance,
        ["LocalActiveTFWeaponData", "m_flObservedCritChance"],
        f32
    );
    define_netvar!(
        get_last_crit_check_time,
        ["LocalActiveTFWeaponData", "m_flLastCritCheckTime"],
        f32
    );
}
