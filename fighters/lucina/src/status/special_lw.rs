use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        special_lw_pre,
        special_lw_main,
    );
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION, hash40("special_lw"));
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR, hash40("special_air_lw"));
    special_lw_main_motion_helper(fighter);
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_motion_helper(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[globals::SITUATION_KIND].get_i32();
    if situation != *SITUATION_KIND_GROUND {
        let mot = VarModule::get_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0);
        }
    } 
    else {
        let mot = VarModule::get_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0);
        }
    }

}

unsafe extern "C" fn special_lw_check_follow_up(fighter: &mut L2CFighterCommon) {
    let stick_y = fighter.global_table[globals::STICK_Y].get_f32();
    let mot;
    let mot_air;
    if stick_y >= 0.5 {
        mot = hash40("special_s4_hi");
        mot_air = hash40("special_air_s4_hi");
    }
    else if stick_y <= -0.5 {
        mot = hash40("special_s4_lw");
        mot_air = hash40("special_air_s4_lw")
    }
    else {
        mot = hash40("special_s4_s");
        mot_air = hash40("special_air_s4_s")
    }
    let stick_x = fighter.global_table[globals::STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if stick_x * lr < -0.33 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION, mot);
    VarModule::set_int64(fighter.battle_object, vars::lucina::status::SPECIAL_LW_MOTION_AIR, mot_air);
    println!("call from check follow up");
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    special_lw_main_motion_helper(fighter);
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK)
        && fighter.global_table[globals::PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            special_lw_check_follow_up(fighter);
            VarModule::off_flag(fighter.battle_object,vars::lucina::status::SPECIAL_LW_SPECIAL_CHECK);
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_lw_main_motion_helper(fighter);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

