use super::*;

pub unsafe extern "C" fn special_lw2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if super::special_check_summon(fighter) {
        return 1.into();
    }

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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0,
    );

    0.into()
}

pub unsafe extern "C" fn special_lw2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if super::dispatch_arsene(fighter) {
    super::dispatch_arsene(fighter);
    let gauge = VarModule::get_float(
        fighter.object(),
        vars::jack::instance::REBEL_GAUGE_ON_SUMMON_DISPATCH,
    );
    fighter.set_float(gauge, 0x4D);

    // for customize_to in [
    //     *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1,
    //     *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
    //     *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1,
    // ] {
    //     fighter.set_int(
    //         customize_to,
    //         *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO,
    //     );
    //     super::jack_move_customizer(fighter);
    // }

    fighter.change_status(FIGHTER_JACK_STATUS_KIND_DISPATCH.into(), false.into());
    return 1.into();
    // }
    // 0.into()
}

pub unsafe extern "C" fn special_lw2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
