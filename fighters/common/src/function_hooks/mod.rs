use super::*;
pub mod change_motion;
pub mod change_status;
pub mod controls;
pub mod directional_influence;
pub mod djcancel;
pub mod edge_slipoffs;
pub mod effect;
pub mod energy;
pub mod get_param;
pub mod hitstun;
pub mod init_settings;
pub mod is_flag;
pub mod jumps;
pub mod ledges;
pub mod misc;
pub mod momentum_transfer;
pub mod stage_hazards;
pub mod transition;

pub fn install() {
    energy::install();
    effect::install();
    edge_slipoffs::install();
    ledges::install();
    get_param::install();
    change_motion::install();
    transition::install();
    djcancel::install();
    init_settings::install();
    hitstun::install();
    change_status::install();
    is_flag::install();
    controls::install();
    momentum_transfer::install();
    jumps::install();
    stage_hazards::install();
    misc::install();

    unsafe {
        // Handles getting rid of the kill zoom
        const NOP: u32 = 0xD503201Fu32;
        skyline::patching::patch_data(utils::offsets::kill_zoom_regular(), &NOP);
        skyline::patching::patch_data(utils::offsets::kill_zoom_throw(), &NOP);
        // Changes full hops to calculate vertical velocity identically to short hops
        skyline::patching::patch_data(0x6d2188, &0x52800015u32);
    }
}
