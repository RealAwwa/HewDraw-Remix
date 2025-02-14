// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Jump during Spin Turn
unsafe fn sonic_spindash_jump_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32){
if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN && boma.is_input_jump() {
    StatusModule::change_status_request_from_script(boma, *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP, true);
  }
}

// upB freefalls after one use per airtime
unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_situation(*SITUATION_KIND_GROUND)
        || fighter.is_situation(*SITUATION_KIND_CLIFF)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]))
    {
        VarModule::off_flag(fighter.battle_object, vars::sonic::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }
    if fighter.is_prev_status(*FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::sonic::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }
    if fighter.is_status(*FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && VarModule::is_flag(fighter.battle_object, vars::sonic::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                let accel_x_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_accel_x_mul");
                let speed_x_max_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.fall_special_speed_x_max_mul");
                WorkModule::set_float(fighter.module_accessor, accel_x_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MUL_FALL_X_ACCEL);
                WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
                *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_REBOUND,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP,
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
    sonic_spindash_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    //sonic_moveset(boma, situation_kind, status_kind, motion_kind, frame, cat[0], id);
    //sonic_lightspeed_dash(boma, status_kind, motion_kind, situation_kind, cat[0], id);
    up_special_freefall(fighter);
    fastfall_specials(fighter);
    
}

pub extern "C" fn sonic_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		sonic_frame(fighter)
    }
}

pub unsafe fn sonic_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, sonic_frame_wrapper);
}
