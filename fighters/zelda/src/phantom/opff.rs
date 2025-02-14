// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn phantom_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    GroundModule::correct(weapon.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let zelda_boma = &mut *(*zelda).module_accessor;
    //check if phantom landed a hit
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(zelda, vars::zelda::instance::SPECIAL_LW_PHANTOM_HIT); 
    } else if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR) {
        AttackModule::clear_inflict_kind_status(weapon.module_accessor);
        StatusModule::change_status_force(weapon.module_accessor, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, false);
    }
    //apply death timer to destroyed phantom if it didn't land a clean hit
    if StopModule::is_stop(weapon.module_accessor) && !VarModule::is_flag(zelda, vars::zelda::instance::SPECIAL_LW_PHANTOM_HIT) {
        VarModule::on_flag(zelda, vars::zelda::instance::SPECIAL_LW_DISABLE_PHANTOM);
    }
    //upon release flash on zelda's hand
    if weapon.is_status(*WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK) && StatusModule::is_changing(weapon.module_accessor) {
        EFFECT_FOLLOW(get_fighter_common_from_accessor(zelda_boma), Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.45, true);
        LAST_EFFECT_SET_COLOR(get_fighter_common_from_accessor(zelda_boma), 0.4, 0.0, 1.0);
    }
    //misc mechanics
    if weapon.is_status(*WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD) {
        let remaining_hitstun = WorkModule::get_float(zelda_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        if weapon.is_situation(*SITUATION_KIND_AIR) {
            let through_passable_ground_stick_y= WorkModule::get_param_float(zelda_boma, hash40("common"), hash40("through_passable_ground_stick_y")) * -1.0;
            if zelda_boma.stick_y() < through_passable_ground_stick_y {
                GroundModule::set_passable_check(weapon.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(weapon.module_accessor, false);
            }
        }
        if zelda_boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
            *FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_DASH,
            *FIGHTER_STATUS_KIND_CATCH_TURN,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
            *FIGHTER_STATUS_KIND_CATCH_WAIT,
            *FIGHTER_STATUS_KIND_CATCH_ATTACK,
            *FIGHTER_STATUS_KIND_CATCH_CUT,
            *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_CHARGE,
            *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_END,
            *FIGHTER_STATUS_KIND_THROW
        ])
            || WorkModule::is_flag(zelda_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
            || WorkModule::is_flag(zelda_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
            || remaining_hitstun > 0.0
        {
            return
        }

        if AttackModule::is_infliction_status(zelda_boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !AttackModule::is_infliction_status(zelda_boma, *COLLISION_KIND_MASK_PARRY)
        && zelda_boma.is_cat_flag(Cat1::SpecialLw) {
            let sound = SoundModule::play_se(zelda_boma, Hash40::new("se_zelda_special_l09"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(zelda_boma, sound as i32, 1.6, 0);
            StatusModule::change_status_force(weapon.module_accessor, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK, false);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, phantom_callback);
}
