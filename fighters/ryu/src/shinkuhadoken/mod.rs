use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("ryu_shinkuhadoken");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}