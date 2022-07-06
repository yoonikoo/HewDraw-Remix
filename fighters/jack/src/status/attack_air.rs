use super::*;

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if super::hit_cancel(fighter) {
        return 1.into();
    }

    fighter.status_AttackAir_Main()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    fighter.main_shift(attack_air_main_loop)
}

pub fn install() {
    smashline::install_status_scripts!(attack_air_main);
}
