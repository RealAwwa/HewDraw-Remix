use super::*;
use globals::*;

//=================================================================
//== init_settings for edge_slipoffs module
//== Note: This is called from init_settings::init_settings_hook
//== Note: Forces GroundModule::correct to be called for
//         certain statuses
//== Note: JostleModule::set_team(boma, 0) is for walking through
//         other fighters
//=================================================================
pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
                              ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
                              arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
    /* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);

    if boma.is_fighter()
    && boma.is_situation(*SITUATION_KIND_GROUND) {

        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            fix = *GROUND_CORRECT_KIND_GROUND as u32; /* GROUND_CORRECT_KIND_GROUND is i32 */
        }

        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT_F,
            *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            // *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            // *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
            *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
            *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
            *FIGHTER_STATUS_KIND_SAVING_DAMAGE].contains(&status_kind) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }

        if    (fighter_kind == *FIGHTER_KIND_EDGE && [*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind) && StatusModule::prev_status_kind(boma, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
           || (fighter_kind == *FIGHTER_KIND_KAMUI && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_MIIFIGHTER && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_SZEROSUIT && [*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_LANDING,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_BAYONETTA && [*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D,
                                                           *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_DOLLY && [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind)) 
           || (boma.kind() == *FIGHTER_KIND_KOOPAJR && boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK,
                                                       *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind))
           || (boma.kind() == *FIGHTER_KIND_LUIGI && boma.is_status(*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL))
           || (boma.kind() == *FIGHTER_KIND_PIT && boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_LANDING))
           || (boma.kind() == *FIGHTER_KIND_PITB && boma.is_status(*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_LANDING))
           || (boma.kind() == *FIGHTER_KIND_SIMON && boma.is_status(*FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING))
           || (boma.kind() == *FIGHTER_KIND_RICHTER && boma.is_status(*FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING))
           || (boma.kind() == *FIGHTER_KIND_DEMON && boma.is_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_LANDING))
           || (boma.kind() == *FIGHTER_KIND_RYU && boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_KEN && boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_PACKUN && boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_KROOL && boma.is_status(*FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_PIKMIN && boma.is_status(*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_FALCO && boma.is_status(*FIGHTER_FALCO_STATUS_KIND_SPECIAL_S_FALL_LANDING))
           || (boma.kind() == *FIGHTER_KIND_MURABITO && boma.is_status(*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_NESS && boma.is_status(*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END))
           || (boma.kind() == *FIGHTER_KIND_LUCAS && boma.is_status(*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END))
        {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }
    }
    return fix
}

//=================================================================
//== GroundModule::correct
//== Note: This is the "can edge slippoff" function in Smash
//=================================================================
#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {

    // don't run if boma is not fighter or grounded
    if !boma.is_fighter() || !boma.is_situation(*SITUATION_KIND_GROUND) {
        return original!()(boma, kind);
    }

    let status_kind = StatusModule::status_kind(boma);
    if [
        // common statuses that should edge slipoff
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
        *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP,
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END,
        *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE
    ].contains(&status_kind)
    || check_fighter_edge_slipoffs(boma).get_bool() {
        return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    original!()(boma, kind)
}

unsafe fn check_fighter_edge_slipoffs(boma: &mut BattleObjectModuleAccessor) -> L2CValue {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = boma.kind();

    // PIKACHU & PICHU
    if (fighter_kind == *FIGHTER_KIND_PIKACHU || fighter_kind == *FIGHTER_KIND_PICHU) 
    && [
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END
    ].contains(&status_kind) {
        return true.into();
    }
    
    // CAPTAIN FALCON
    if fighter_kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END { return true.into(); }
    
    // GANONDORF
    if (fighter_kind == *FIGHTER_KIND_GANON && status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END) { return true.into(); }
    
    // MII SWORDFIGHTER
    if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN
    && (
        [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END].contains(&status_kind) 
        || (
            WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 
            && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
        )
    ) { 
        return true.into(); 
    }
    
    // BOWSER
    if (fighter_kind == *FIGHTER_KIND_KOOPA && status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G) { return true.into(); }

    // DONKEY KONG
    if (fighter_kind == *FIGHTER_KIND_DONKEY && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI) { return true.into(); }

    // GAOGAEN
    if (fighter_kind == *FIGHTER_KIND_GAOGAEN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N) { return true.into(); }
    if (fighter_kind == *FIGHTER_KIND_KIRBY && status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N) { return true.into(); }

    // LUIGI
    if (fighter_kind == *FIGHTER_KIND_LUIGI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N) { return true.into(); }
    if (fighter_kind == *FIGHTER_KIND_KIRBY && status_kind == *FIGHTER_KIRBY_STATUS_KIND_LUIGI_SPECIAL_N) { return true.into(); }

    // PEACH
    if (fighter_kind == *FIGHTER_KIND_PEACH && status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END) { return true.into(); }

    // DAISY
    if (fighter_kind == *FIGHTER_KIND_DAISY && status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_AWAY_END) { return true.into(); }

    // SEPHIROTH
    if (fighter_kind == *FIGHTER_KIND_EDGE && status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH) { return true.into(); }
    
    // YOSHI
    if (fighter_kind == *FIGHTER_KIND_YOSHI && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI) { return true.into(); }
    
    // PAC-MAN
    if (fighter_kind == *FIGHTER_KIND_PACMAN && status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN) { return true.into(); }
    
    // SORA
    if (fighter_kind == *FIGHTER_KIND_TRAIL && status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END) { return true.into(); }
    
    // CHARIZARD
    if (fighter_kind == *FIGHTER_KIND_PLIZARDON && status_kind == *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END) { return true.into(); }

    //KING DEDEDE
    if fighter_kind == *FIGHTER_KIND_DEDEDE 
    && [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK
    ].contains(&status_kind){
            return true.into();
    }
    
    // FOX
    if fighter_kind == *FIGHTER_KIND_FOX 
    && [
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END
    ].contains(&status_kind) { 
        return true.into();
    }
    
    // WOLF
    if fighter_kind == *FIGHTER_KIND_WOLF 
    && [
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, 
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 
        *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END
    ].contains(&status_kind) { 
        return true.into(); 
    }
    
    // NESS
    if fighter_kind == *FIGHTER_KIND_NESS 
    && [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
    ].contains(&status_kind) {
        return true.into();
    }

    // LUCAS
    if fighter_kind == *FIGHTER_KIND_LUCAS 
    && [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END
    ].contains(&status_kind) {
        return true.into();
    }

    return false.into();
}

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut BattleObjectModuleAccessor, something: i32) -> i32;
}

//=================================================================
//== FighterUtil::get_ground_correct_kind_air_trans
//== Note: Aerial ECB fixes for Link, Captain, Simon, Richter
//=================================================================
#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe fn get_ground_correct_kind_air_trans_hook(boma: &mut BattleObjectModuleAccessor, something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

//=================================================================
//== GroundModule::can_entry_cliff
//== Note: Handles ledgehogging, fighter-specific ledge entry,
//         and disallows rising ledge grabs (for non-specials)
//=================================================================
#[skyline::hook(replace=GroundModule::can_entry_cliff)]
unsafe fn can_entry_cliff_hook(boma: &mut BattleObjectModuleAccessor) -> u64 {
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = boma.kind();

    let rising: f32 = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); // Rising while jumping/airdodging

    let tether_zair = boma.is_fighter()
                        && [*FIGHTER_KIND_LUCAS, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_SZEROSUIT].contains(&fighter_kind)
                        && [*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND].contains(&status_kind);

    let tether_special = boma.is_fighter()
                        && ( (fighter_kind == *FIGHTER_KIND_SZEROSUIT   && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
                          || (fighter_kind == *FIGHTER_KIND_SHIZUE      && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
                          || (fighter_kind == *FIGHTER_KIND_TANTAN      && (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR))
                          || (fighter_kind == *FIGHTER_KIND_MASTER      && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
                          || (fighter_kind == *FIGHTER_KIND_JACK        && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
                          || (fighter_kind == *FIGHTER_KIND_PFUSHIGISOU && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI) );

    let tether_aerial = boma.is_fighter()
                        && ( (fighter_kind == *FIGHTER_KIND_SIMON   && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR) );

    // Ledgehog code
    let cliff_id = GroundModule::get_cliff_id_uint32(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    for object_id in util::get_all_active_battle_object_ids() {
        let object = ::utils::util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                continue;
            }

            if VarModule::get_int(object, vars::common::instance::LEDGE_ID) == cliff_id as i32 {
                if !((tether_zair || tether_special || tether_aerial) && WorkModule::is_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)) {
                    return 0;
                }
            }
        }
    }

    if boma.is_fighter() {
        if !run_vanilla_check(boma) {
            // Disable grabbing ledge while rising during an airborne state
            if situation_kind == *SITUATION_KIND_AIR {
                if rising >= 0.0 && !((tether_zair || tether_special || tether_aerial) && WorkModule::is_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)) {
                    return 0;
                }
            }
        }

        // Unable to grab ledge during runfall/walkfall (the first few frames after you run off an edge)
        if boma.is_motion_one_of(&[Hash40::new("run_fall_l"), Hash40::new("run_fall_r"), Hash40::new("walk_fall_l"), Hash40::new("walk_fall_r")]) {
            return 0;
        }
    }

    original!()(boma)
}

unsafe fn run_vanilla_check(boma: &mut BattleObjectModuleAccessor) -> bool {
    if boma.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
        return true;
    }

    if [*FIGHTER_KIND_ICE_CLIMBER, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&boma.kind())
    && boma.is_status_one_of(&[
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PRE,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_PARTNER,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PARTNER,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_COMP_PARTNER,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_PULL_PARTNER,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_COMP])
    {
        return true;
    }

    false
}

pub fn install() {
    skyline::install_hooks!(
        correct_hook,
        get_ground_correct_kind_air_trans_hook,
        can_entry_cliff_hook
    );
}