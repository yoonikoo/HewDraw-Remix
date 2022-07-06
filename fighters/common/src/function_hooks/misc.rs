use super::*;

#[skyline::hook(offset = 0xb35edc, inline)]
unsafe fn jack_null_regular_damage(ctx: &skyline::hooks::InlineCtx) {
    std::arch::asm!("fmov s8, 0.0");
}

#[skyline::hook(replace = smash::app::FighterSpecializer_Jack::add_rebel_gauge)]
pub unsafe fn add_rebel_gauge(
    boma: &mut app::BattleObjectModuleAccessor,
    entry_id: i32,
    amount: f32,
) {
    if !boma.is_flag(0x200000e9) {
        // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE
        return;
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_STANDBY,
    ]) {
        return;
    }

    if boma.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        let customize = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO);
        if customize != 0 {
            return;
        }
    }

    if boma.is_flag(0x200000e7) {
        // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUSPEND
        return;
    }

    if boma.is_flag(0x200000e3) {
        // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUMMON
        return;
    }

    if boma.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
        return;
    }

    let current_gauge = boma.get_float(0x4D); // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    let new_gauge = current_gauge + amount;
    if new_gauge >= 100.0 {
        boma.set_float(100.0, 0x4D); // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    } else {
        boma.set_float(new_gauge.max(0.0), 0x4D); // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    }

    smash2::app::FighterManager::instance().unwrap().send_event(
        smash2::app::JackUpdateRebelGaugeEvent::new(
            entry_id as u32,
            new_gauge.clamp(0.0, 100.0) / 100.0,
        ),
    );
}

pub fn install() {
    skyline::install_hooks!(jack_null_regular_damage, add_rebel_gauge);
}
