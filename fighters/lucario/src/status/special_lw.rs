use super::*;
use crate::opff::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return 0.into();
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    special_lw_set_kinetic(fighter);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::on_flag(fighter.object(), vars::lucario::instance::DISABLE_SPECIAL_LW);
    }
    ControlModule::clear_command(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            // landing transition
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.set_float(10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 0.into();
            }
            special_lw_set_kinetic(fighter);
            return 0.into();
        }
    }
    // check for cancels
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) 
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) 
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    } else {
        if special_lw_check_cancel(fighter).get_bool() {
            return true.into();
        }
    }
    // end
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into())
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    if !MotionModule::is_end(fighter.module_accessor)
    && !CancelModule::is_enable_cancel(fighter.module_accessor)
    && [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        // *FIGHTER_STATUS_KIND_SPECIAL_LW,
    ].contains(&next_status) {
        MeterModule::drain_direct(fighter.object(), MeterModule::meter_per_level(fighter.object()));
        opff::check_burnout(fighter);
        pause_meter_regen(fighter, 120);
        if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.7, y: 0.7, z: 0.7}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_check_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    || fighter.is_in_hitlag() 
    || VarModule::is_flag(fighter.object(), vars::lucario::instance::METER_BURNOUT)
    || !VarModule::is_flag(fighter.battle_object, vars::lucario::status::HIT_CANCEL) {
        return false.into();
    }
    if fighter.is_cat_flag(Cat1::SpecialN) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
        return true.into();
    }
    if fighter.is_cat_flag(Cat1::SpecialS)
    && !fighter.is_cat_flag(Cat1::SpecialLw) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
        return true.into();
    }
    if fighter.is_cat_flag(Cat1::SpecialHi) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
        return true.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.is_cat_flag(Cat1::Catch) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH,true);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackS4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackHi4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackLw4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackS3) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackHi3) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackLw3) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
            return true.into();
        }
        if fighter.is_cat_flag(Cat1::AttackN) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK,true);
            return true.into();
        }
    } else {
        if fighter.get_aerial() != None {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR,true);
            return true.into();
        }
    }
    return false.into();
}

unsafe extern "C" fn special_lw_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
}