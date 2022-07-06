use super::*;

pub mod attack3;
pub mod attack_air;
pub mod doyle;
pub mod special_lw;
pub mod special_lw2;
pub mod summon;

pub unsafe fn hit_cancel(fighter: &mut L2CFighterCommon) -> bool {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !fighter.is_in_hitlag()
        && fighter.is_cat_flag(Cat1::SpecialLw)
        && fighter.get_float(0x4D)
            > ParamModule::get_float(
                fighter.object(),
                ParamType::Agent,
                "rebel_gauge.cancel_require",
            )
        && !CancelModule::is_enable_cancel(fighter.module_accessor)
        && !fighter.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)
    {
        let keep_mul = ParamModule::get_float(
            fighter.object(),
            ParamType::Agent,
            "rebel_gauge.cancel_keep_mul",
        );
        WorkModule::mul_float(fighter.module_accessor, keep_mul, 0x4D);
        summon_arsene(fighter);
        VarModule::on_flag(fighter.object(), vars::jack::status::SUMMON_FROM_CANCEL);
        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SUMMON.into(), true.into());
        true
    } else {
        false
    }
}

pub unsafe fn special_check_summon(fighter: &mut L2CFighterCommon) -> bool {
    if !fighter.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESERVE_SUMMON_DISPATCH)
        && fighter.get_int(*FIGHTER_JACK_INSTANCE_WORK_ID_INT_CUSTOMIZE_TO) < 0
    {
        return false;
    }

    let interrupt = fighter.global_table[globals::STATUS_KIND_INTERRUPT].get_i32();
    fighter.set_int(
        interrupt,
        *FIGHTER_JACK_INSTANCE_WORK_ID_INT_SPECIAL_KIND_CUSTOMIZE,
    );
    StatusModule::set_status_kind_interrupt(
        fighter.module_accessor,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_CUSTOMIZE,
    );
    true
}

pub unsafe fn try_interrupt_with_summon(fighter: &mut L2CFighterCommon) -> bool {
    if summon_arsene(fighter) {
        StatusModule::set_status_kind_interrupt(
            fighter.module_accessor,
            *FIGHTER_JACK_STATUS_KIND_SUMMON,
        );
        true
    } else {
        false
    }
}

pub unsafe fn summon_arsene(fighter: &mut L2CFighterCommon) -> bool {
    let rebel_gauge = fighter.get_float(0x4D);
    fighter.on_flag(0x200000e3);
    VarModule::set_float(
        fighter.battle_object,
        vars::jack::instance::REBEL_GAUGE_ON_SUMMON_DISPATCH,
        rebel_gauge,
    );
    let status_kind = app::FighterSpecializer_Jack::check_doyle_summon_dispatch(
        fighter.module_accessor,
        true,
        true,
    ) as i32;
    if status_kind == *FIGHTER_JACK_STATUS_KIND_SUMMON {
        true
    } else {
        false
    }
}

#[skyline::from_offset(0xb2f800)]
unsafe fn disable_doyle(jack_boma: *mut app::BattleObjectModuleAccessor, arg: u32);

pub unsafe fn dispatch_arsene(fighter: &mut L2CFighterCommon) {
    fighter.on_flag(0x200000e4); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_END
    let _ = app::FighterSpecializer_Jack::check_doyle_summon_dispatch(
        fighter.module_accessor,
        true,
        false,
    );
    disable_doyle(fighter.module_accessor, 0);
}

unsafe fn set_move_customizer(
    fighter: &mut L2CFighterCommon,
    customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue,
) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }

    let clone = fighter.global_table[globals::WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&L2CValue::Bool(true));
    fighter.global_table["move_customizer_original"].assign(&clone);
    fighter.global_table[globals::WAZA_CUSTOMIZE_CONTROL]
        .assign(&L2CValue::Ptr(customizer as *const () as _));
}

unsafe fn get_original_customizer(
    fighter: &mut L2CFighterCommon,
) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } else {
        None
    }
}

unsafe extern "C" fn jack_move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO,
    );
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw::special_lw_pre as *const ()),
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_lw::special_lw_main as *const ()),
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_lw::special_lw_end as *const ()),
        );
        0.into()
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw2::special_lw2_pre as *const ()),
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_lw2::special_lw2_main as *const ()),
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_lw2::special_lw2_end as *const ()),
        );
        0.into()
    } else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } else {
        0.into()
    }
}

#[fighter_init]
fn jack_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_JACK {
            return;
        }

        set_move_customizer(fighter, jack_move_customizer);
        jack_move_customizer(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(jack_init);
    attack_air::install();
    attack3::install();
    doyle::install();
    special_lw::install();
    summon::install();
}
