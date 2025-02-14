// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn teleport_tech(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && !VarModule::is_flag(fighter.battle_object, vars::zelda::instance::SPECIAL_HI_GROUNDED_TELEPORT) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            VarModule::on_flag(fighter.battle_object, vars::zelda::instance::SPECIAL_HI_GROUNDED_TELEPORT)
        } //touching ground at any point counts as G2G for cancels
    }
    // Wall Ride momentum fixes
    //if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2) {
        //let init_speed_x = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_X);
        //let init_speed_y = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y);
        //if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
        //    if !VarModule::is_flag(boma.object(), vars::common::status::IS_TELEPORT_WALL_RIDE) {
        //        VarModule::on_flag(boma.object(), vars::common::status::IS_TELEPORT_WALL_RIDE);
        //    }
        //    if init_speed_y > 0.0 {
        //        fighter.clear_lua_stack();
        //        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
        //        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        //    }
        //} else if VarModule::is_flag(boma.object(), vars::common::status::IS_TELEPORT_WALL_RIDE) {
        //    fighter.clear_lua_stack();
        //    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, init_speed_x, init_speed_y);
        //    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        //}
        //telecancel
    //    if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
    //        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    //        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), true.into());
    //        return;
    //    }
    //}
    //else if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3) {   
    //    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
    //        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
    //            let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
    //            KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    //        }
    //    }
    //}
}

unsafe fn phantom_special_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
    && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_PARRY)
    && !fighter.is_in_hitlag()
    && VarModule::is_flag(fighter.battle_object, vars::zelda::status::SPECIAL_LW_PHANTOM_CANCEL_FRAME) {
        if fighter.is_cat_flag(Cat1::SpecialLw) && !ArticleModule::is_exist(boma, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM) {
            if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) { //displacement flag
                VarModule::on_flag(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_FORWARD_PHANTOM);
            }//cancel if phantom is off cd
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
}

unsafe fn nayru_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("special_n")) 
    && StatusModule::is_situation_changed(boma)
    && MotionModule::frame(boma) < 55.0 {
        EffectModule::kill_kind(boma, Hash40::new("zelda_nayru_l"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("zelda_nayru_r"), true, true);
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_n"), 56.0, 1.0, 1.0);
        AttackModule::clear_all(boma);
        boma.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
    }
}

/// Handles land canceling when airborne
unsafe fn dins_fire_cancels(boma: &mut BattleObjectModuleAccessor){
    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                WorkModule::set_float(boma, 7.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                boma.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
            }
        }
    }
}

pub unsafe fn phantom_usability_effects(fighter:&mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let phantom_object_id = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_PHANTOM_OBJECT_ID) as u32;
    let phantom_battle_object = utils::util::get_battle_object_from_id(phantom_object_id);
    let phantom_boma = &mut *(*phantom_battle_object).module_accessor;
    let handle = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE);
    let arrow = VarModule::get_int(phantom_battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE);
    //disables effects on winscreen (one of them spawns a phantom)
    if (fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY]) || !sv_information::is_ready_go())  && handle >= 1 {
        EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_aura"), true, true);
        VarModule::set_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE, -2);
    } else {
        if ArticleModule::is_exist(boma, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM) {
            if !EffectModule::is_exist_effect(boma, handle as u32) && handle > -1 {
                VarModule::set_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE, -1);
            } //resets effect
            if handle == -1 {
                let handle = EffectModule::req_follow(boma, Hash40::new("zelda_phantom_aura"), Hash40::new("havel"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 1.05, true, 0, 0, 0, 0, 0, true, true) as u32;
                VarModule::set_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE, handle as i32);
            }//if phantom spawned and effects are enabled
        } else {
            if handle >= 0 { //when phantom dies play effect then shift to clear flags
                EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.8, true);
                app::FighterUtil::flash_eye_info(fighter.module_accessor);
                EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_aura"), true, true);
                VarModule::off_flag(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_DISABLE_PHANTOM);
                VarModule::off_flag(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_PHANTOM_HIT);
                VarModule::set_int(fighter.battle_object, vars::zelda::instance::SPECIAL_LW_COOLDOWN_EFFECT_HANDLE, -1);
            }//-1 allows effects to be spawned
            if EffectModule::is_exist_effect(phantom_boma, arrow as u32) {
                EffectModule::kill(phantom_boma, arrow as u32, true, true);
            }//kill check for player arrow
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    teleport_tech(fighter, boma, frame);
    dins_fire_cancels(boma);
    nayru_land_cancel(boma);
    phantom_special_cancel(fighter, boma);
    phantom_usability_effects(fighter, boma);
    fastfall_specials(fighter);
}

pub extern "C" fn zelda_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		zelda_frame(fighter)
    }
}

pub unsafe fn zelda_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, zelda_frame_wrapper);
}
