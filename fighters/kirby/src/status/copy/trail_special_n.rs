use super::*;

// FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2

unsafe extern "C" fn special_n2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0,
    );
    return 0.into();
}

unsafe extern "C" fn special_n2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("trail_special_n2"), L2CValue::Hash40s("trail_special_air_n2"), false.into());
    let initial_speed_y = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        Some(app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent))
    }
    else {
        None
    };
    // Added code
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    if situation_kind == *SITUATION_KIND_AIR {
        // Glide a small amount in the air unless there's too much positive y energy (to avoid flying to the top blastzone)
        let mut aerial_y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mut aerial_x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * 0.001;
        // let mut x_string = aerial_x_speed.to_string();
        // let mut y_string = aerial_y_speed.to_string();
        // println!("Pre X: {}" , x_string);
        // println!("Pre Y: {}" , y_string);
        let mut reset_speed_2f = Vector2f { x: aerial_x_speed, y: 0.0 };
        let mut reset_speed_gravity_2f = Vector2f { x: 0.0, y: 0.0 };
        let mut reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
        lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_gravity_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::enable(stop_energy);
        lua_bind::KineticEnergy::enable(gravity_energy);
        // Don't allow drift during the move and set accelleration to slow descent
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        lua_bind::FighterKineticEnergyGravity::set_accel(fighter.get_gravity_energy(), -0.02);
        lua_bind::FighterKineticEnergyGravity::set_gravity_coefficient(fighter.get_gravity_energy(), 0.7);
        // Keep accelleration
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, aerial_x_speed);
        // Bounds here were made based off testing, may need tweaking if not getting float effect
        // when expected or vice versa
        if aerial_y_speed >= 0.9 && (aerial_x_speed <= 0.0011 && aerial_x_speed >= -0.0011)  {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            fighter.sub_set_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_n"));
        }
    }  
    // End of added code 
    if let Some(speed) = initial_speed_y {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed);
        Some(app::sv_kinetic_energy::set_speed(fighter.lua_state_agent));
    }
    fighter.sub_set_ground_correct_by_situation(true.into());
    return fighter.main_shift(special_n2_main_loop);
}

unsafe extern "C" fn special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !(CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool()
        || fighter.sub_air_check_fall_common().get_bool()) {
        if !(app::lua_bind::MotionModule::is_end(fighter.module_accessor)) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("trail_special_n2"), L2CValue::Hash40s("trail_special_air_n2"), true.into());
            fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_n"));
            fighter.sub_set_ground_correct_by_situation(true.into());
            special_n2_main_loop_function(fighter, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_SHOOTED, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_SPECIAL_N2_HOP);
            return 0.into();
        }
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_n2_main_loop_function(fighter: &mut L2CFighterCommon, flag_shooted: i32, work_id_n2_hop: i32) {
    if WorkModule::is_flag(fighter.module_accessor, flag_shooted) {
        WorkModule::off_flag(fighter.module_accessor, flag_shooted);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_AIR
        && WorkModule::is_flag(fighter.module_accessor, work_id_n2_hop) {
            WorkModule::on_flag(fighter.module_accessor, work_id_n2_hop);
            let x_param_float = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("hop_add_speed_x"));
            let posture_module_lr = app::lua_bind::PostureModule::lr(fighter.module_accessor);
            let inertia = x_param_float * posture_module_lr;
            let y_param_float = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("param_special_n"), smash::hash40("hop_add_speed_y"));
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, inertia, 0.0, 0.0, 0.0, 0.0);
            app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_param_float);
        }
    }

    // allow the move to be turned around
    if fighter.is_status (*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2)
    && fighter.status_frame() >= 5
    && fighter.is_button_on(Buttons::Special)
    && fighter.stick_x() * fighter.lr() < 0.0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    return;
}

// reimplemented end statuses to remove the native magic switching. this is now tied to a timer in opff

unsafe extern "C" fn special_n1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_SHOOT {
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::off_flag(fighter.module_accessor,  *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
        VarModule::set_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER, MAGIC_COOLDOWN_FRAME);
    }

    return 0.into()
}

unsafe extern "C" fn special_n1_shoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_END {
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::off_flag(fighter.module_accessor,  *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
        VarModule::set_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER, MAGIC_COOLDOWN_FRAME);
    }

    return 0.into()
}

unsafe extern "C" fn special_n1_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    WorkModule::off_flag(fighter.module_accessor,  *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    VarModule::set_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER, MAGIC_COOLDOWN_FRAME);

    return 0.into()
}

unsafe extern "C" fn special_n2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    WorkModule::off_flag(fighter.module_accessor,  *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    VarModule::set_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER, MAGIC_COOLDOWN_FRAME);

    return 0.into()
}

unsafe extern "C" fn special_n3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_CLOUD, ArticleOperationTarget(0));
    WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    WorkModule::off_flag(fighter.module_accessor,  *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    VarModule::set_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER, MAGIC_COOLDOWN_FRAME);

    return 0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2, special_n2_pre);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2, special_n2_main);

    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1, special_n1_end);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_SHOOT, special_n1_shoot_end);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_END, special_n1_end_end);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2, special_n2_end);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3, special_n3_end);
}
