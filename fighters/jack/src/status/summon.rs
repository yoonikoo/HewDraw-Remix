use super::*;

utils::import_noreturn!(common::opff::check_b_reverse);

#[status_script(agent = "jack", status = FIGHTER_JACK_STATUS_KIND_SUMMON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn summon_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.object(), vars::jack::status::SUMMON_FROM_CANCEL);

    let kinetic = if is_cancel {
        *FIGHTER_KINETIC_TYPE_RESET
    } else {
        *FIGHTER_KINETIC_TYPE_UNIQ
    };

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        kinetic,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );

    VarModule::set_flag(
        fighter.object(),
        vars::jack::status::SUMMON_FROM_CANCEL,
        is_cancel,
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0,
    );

    0.into()
}

unsafe extern "C" fn summon_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor)
        && fighter.global_table[globals::SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        GroundModule::correct(
            fighter.module_accessor,
            app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND),
        );
    }

    common::opff::check_b_reverse(fighter);

    if VarModule::is_flag(fighter.object(), vars::jack::status::SUMMON_FROM_CANCEL)
        && VarModule::countdown_int(
            fighter.object(),
            vars::jack::status::CANCEL_SUMMON_CANCEL_FRAME,
            0,
        )
    {
        CancelModule::enable_cancel(fighter.module_accessor);
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter
            .sub_wait_ground_check_common(false.into())
            .get_bool()
            || fighter.sub_air_check_fall_common().get_bool()
        {
            return 1.into();
        }

        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[globals::SITUATION_KIND] == SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }

    0.into()
}

#[status_script(agent = "jack", status = FIGHTER_JACK_STATUS_KIND_SUMMON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn summon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if app::FighterSpecializer_Jack::is_cut_in_effect(fighter.module_accessor) {
        fighter.on_flag(*FIGHTER_JACK_STATUS_SUMMON_FLAG_CUT_IN_EFFECT);
    }

    if fighter.global_table[globals::SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        GroundModule::correct(
            fighter.module_accessor,
            app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND),
        );
    }

    if VarModule::is_flag(fighter.object(), vars::jack::status::SUMMON_FROM_CANCEL) {
        let cancel_frame = ParamModule::get_int(
            fighter.object(),
            ParamType::Agent,
            "rebel_gauge.cancel_cancel_frame",
        );
        VarModule::set_int(
            fighter.object(),
            vars::jack::status::CANCEL_SUMMON_CANCEL_FRAME,
            cancel_frame,
        );
    }

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("summon"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );

    if fighter.is_flag(*FIGHTER_JACK_STATUS_SUMMON_FLAG_CUT_IN_EFFECT) {
        MotionAnimcmdModule::flush(fighter.module_accessor, false);
        app::FighterSpecializer_Jack::set_cut_in_effect(fighter.module_accessor);
    }

    VisibilityModule::set_int64(
        fighter.module_accessor,
        smash::hash40("mask") as _,
        smash::hash40("on") as _,
    );

    VisibilityModule::set_material_anim_priority(
        fighter.module_accessor,
        Hash40::new("mask"),
        true,
    );

    fighter.main_shift(summon_main_loop)
}

#[status_script(agent = "jack", status = FIGHTER_JACK_STATUS_KIND_SUMMON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn summon_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_material_anim_priority(
        fighter.module_accessor,
        Hash40::new("mask"),
        false,
    );

    let xlu_frame = fighter.get_param_int("param_private", "summon_dispatch_xlu_frame");
    HitModule::set_xlu_frame_global(fighter.module_accessor, xlu_frame, 0);

    EffectModule::enable_stencil(fighter.module_accessor, false);

    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(summon_pre, summon_main, summon_end);
}
