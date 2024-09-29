use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_lw;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon){
    VarModule::set_int(fighter.battle_object, vars::dedede::instance::RECATCH_COUNTER, 0);
    VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CONTINUE_JET_SPIN, false);
    VarModule::set_int(fighter.battle_object, vars::dedede::instance::JET_CHARGE_FRAMES, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
}