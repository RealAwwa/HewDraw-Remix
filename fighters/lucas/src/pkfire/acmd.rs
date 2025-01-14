use super::*;

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 60, 10, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.6);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    for i in 1..=50 {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("lucas_pkfr_bullet_ed"), true, true);
            EFFECT_FOLLOW(agent, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(agent, 0.25);
        }
        wait(lua_state, 8.0);
    }
}

unsafe extern "C" fn effect_pillar(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("lucas_pkfr_start"), Hash40::new("lucas_pkfr_start"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        EFFECT(agent, Hash40::new("lucas_pkfr_bomb_max"), Hash40::new("top"), 0, -4.5, -2.7, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_pillar(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) { 
        PLAY_SE_REMAIN(agent, Hash40::new("se_lucas_special_n04_s"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shoot", game_shoot, Priority::Low);
    agent.acmd("effect_shoot", effect_shoot, Priority::Low);

    agent.acmd("game_pillar", acmd_stub, Priority::Low);
    agent.acmd("effect_pillar", effect_pillar, Priority::Low);
    agent.acmd("sound_pillar", sound_pillar, Priority::Low);
}