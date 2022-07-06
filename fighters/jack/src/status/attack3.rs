use super::*;

unsafe extern "C" fn attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if super::hit_cancel(fighter) {
        return 1.into();
    }
    fighter.status_AttackHi3_Main()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let motion_kind = app::sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    std::mem::forget(
        fighter.status_AttackHi3_Common(motion_kind.into(), L2CValue::Hash40s("attack_hi3")),
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }

    fighter.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(
        L2CFighterCommon_sub_attack3_uniq_check as *const () as _,
    ));
    fighter.main_shift(attack_hi3_main_loop)
}

unsafe extern "C" fn attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if super::hit_cancel(fighter) {
        return 1.into();
    }
    fighter.status_AttackLw3_Main()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    std::mem::forget(fighter.status_AttackLw3_common());
    fighter.main_shift(attack_lw3_main_loop)
}

unsafe extern "C" fn attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if super::hit_cancel(fighter) {
        return 1.into();
    }
    fighter.status_AttackS3_Main()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    std::mem::forget(fighter.status_AttackS3Common());
    fighter.main_shift(attack_s3_main_loop)
}

pub fn install() {
    smashline::install_status_scripts!(attack_hi3_main, attack_lw3_main, attack_s3_main);
}
