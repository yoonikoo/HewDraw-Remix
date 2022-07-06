use super::*;

pub trait MainShift {
    fn main_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue,
    ) -> L2CValue;
}

pub trait FastShift {
    fn fast_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterBase) -> L2CValue,
    ) -> L2CValue;
    fn change_to_custom_status(&mut self, id: i32, clear_cat: bool, common: bool);
}

impl MainShift for L2CFighterCommon {
    fn main_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue,
    ) -> L2CValue {
        unsafe { self.sub_shift_status_main(L2CValue::Ptr(new_main as *const () as _)) }
    }
}

impl FastShift for L2CFighterBase {
    fn fast_shift(
        &mut self,
        new_main: unsafe extern "C" fn(&mut L2CFighterBase) -> L2CValue,
    ) -> L2CValue {
        unsafe { self.fastshift(L2CValue::Ptr(new_main as *const () as _)) }
    }

    fn change_to_custom_status(&mut self, id: i32, clear_cat: bool, common: bool) {
        use crate::CustomStatusModule;

        let kind = if common {
            CustomStatusModule::get_common_status_kind(self.battle_object, id)
        } else {
            CustomStatusModule::get_agent_status_kind(self.battle_object, id)
        };

        unsafe { self.change_status(kind.into(), clear_cat.into()) }
    }
}

pub trait BomaExt {
    // INPUTS
    unsafe fn clear_commands<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T);
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool;
    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn stick_x(&mut self) -> f32;
    unsafe fn stick_y(&mut self) -> f32;
    unsafe fn prev_stick_x(&mut self) -> f32;
    unsafe fn prev_stick_y(&mut self) -> f32;
    unsafe fn is_flick_y(&mut self, sensitivity: f32) -> bool;
    unsafe fn is_input_jump(&mut self) -> bool;
    unsafe fn get_aerial(&mut self) -> Option<AerialKind>;
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f);
    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool;

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool;

    // STATE
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_motion(&mut self, motion: Hash40) -> bool;
    unsafe fn is_motion_one_of(&mut self, motions: &[Hash40]) -> bool;
    unsafe fn status(&mut self) -> i32;

    /// gets the number of jumps that have been used
    unsafe fn get_num_used_jumps(&mut self) -> i32;

    /// gets the max allowed number of jumps for this character
    unsafe fn get_jump_count_max(&mut self) -> i32;
    unsafe fn motion_frame(&mut self) -> f32;
    unsafe fn set_rate(&mut self, motion_rate: f32);
    unsafe fn is_in_hitlag(&mut self) -> bool;

    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32;

    // INSTANCE
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;

    // WORK
    unsafe fn get_int(&mut self, what: i32) -> i32;
    unsafe fn get_float(&mut self, what: i32) -> f32;
    unsafe fn get_int64(&mut self, what: i32) -> u64;
    unsafe fn is_flag(&mut self, what: i32) -> bool;
    unsafe fn set_int(&mut self, value: i32, what: i32);
    unsafe fn set_float(&mut self, value: f32, what: i32);
    unsafe fn set_int64(&mut self, value: i64, what: i32);
    unsafe fn on_flag(&mut self, what: i32);
    unsafe fn off_flag(&mut self, what: i32);
    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32;
    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32;
    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64;

    // ENERGY
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion;
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController;
    // tech/general subroutine
    unsafe fn handle_waveland(&mut self, require_airdodge: bool, change_status: bool) -> bool;
    unsafe fn shift_ecb_on_landing(&mut self);
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn clear_commands<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) {
        let cat = fighter_pad_cmd_flag.into();
        let (cat, bits) = match cat {
            CommandCat::Cat1(cat) => (0, cat.bits()),
            CommandCat::Cat2(cat) => (1, cat.bits()),
            CommandCat::Cat3(cat) => (2, cat.bits()),
            CommandCat::Cat4(cat) => (3, cat.bits()),
            CommandCat::CatHdr(cat) => (4, cat.bits()),
        };

        crate::modules::InputModule::clear_commands(self.object(), cat, bits);
    }

    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat),
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat),
            CommandCat::CatHdr(cat) => CatHdr::new(self).intersects(cat),
        }
    }

    unsafe fn is_pad_flag(&mut self, pad_flag: PadFlag) -> bool {
        PadFlag::from_bits_unchecked(ControlModule::get_pad_flag(self)).intersects(pad_flag)
    }

    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button(self)).intersects(buttons)
    }

    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool {
        !self.is_button_on(buttons)
    }

    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_trigger(self)).intersects(buttons)
    }

    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_release(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button_prev(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool {
        !self.was_prev_button_on(buttons)
    }

    unsafe fn stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_x(self);
    }

    unsafe fn stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_y(self);
    }

    unsafe fn prev_stick_x(&mut self) -> f32 {
        return ControlModule::get_stick_prev_x(self);
    }

    unsafe fn prev_stick_y(&mut self) -> f32 {
        return ControlModule::get_stick_prev_y(self);
    }

    unsafe fn is_input_jump(&mut self) -> bool {
        if self.is_cat_flag(Cat1::Jump) && ControlModule::is_enable_flick_jump(self) {
            WorkModule::set_int(
                self,
                1,
                *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE,
            );
            return true;
        }

        return self.is_cat_flag(Cat1::JumpButton);
    }

    // TODO: Reimplement this check
    unsafe fn is_flick_y(&mut self, sensitivity: f32) -> bool {
        let stick = self.stick_y();
        let p_stick = self.prev_stick_y();

        if sensitivity < 0.0
            && stick < sensitivity
            && (stick < p_stick || self.is_cat_flag(Cat2::FallJump))
        {
            return true;
        }

        if sensitivity > 0.0
            && stick > sensitivity
            && (stick > p_stick || self.is_cat_flag(Cat2::FallJump))
        {
            return true;
        }

        return false;
    }

    /// returns whether or not the stick x is pointed in the "forwards" direction for
    /// a character
    unsafe fn is_stick_forward(&mut self) -> bool {
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x * PostureModule::lr(self) > 0. {
                return true;
            }
        }
        return false;
    }

    /// returns whether or not the stick x is pointed in the "backwards" direction for
    /// a character
    unsafe fn is_stick_backward(&mut self) -> bool {
        let stick_value_x = ControlModule::get_stick_x(self);
        if stick_value_x != 0. {
            if stick_value_x * PostureModule::lr(self) < 0. {
                return true;
            }
        }
        return false;
    }

    unsafe fn get_aerial(&mut self) -> Option<AerialKind> {
        if self.is_cat_flag(Cat1::AttackHi3 | Cat1::AttackHi4) {
            Some(AerialKind::Uair)
        } else if self.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackLw4) {
            Some(AerialKind::Dair)
        } else if self.is_cat_flag(Cat1::AttackS3 | Cat1::AttackS4) {
            if self.is_stick_backward() {
                Some(AerialKind::Bair)
            } else {
                Some(AerialKind::Fair)
            }
        } else if self.is_cat_flag(Cat1::AttackN | Cat1::AttackAirN) {
            Some(AerialKind::Nair)
        } else {
            None
        }
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }

    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return StatusModule::prev_status_kind(self, 0) == kind;
    }

    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::prev_status_kind(self, 0);
        return kinds.contains(&kind);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }

    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool {
        return StatusModule::prev_situation_kind(self) == kind;
    }

    unsafe fn is_motion(&mut self, kind: Hash40) -> bool {
        return MotionModule::motion_kind(self) == kind.hash;
    }

    unsafe fn set_rate(&mut self, motion_rate: f32) {
        MotionModule::set_rate(self, motion_rate);
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&Hash40::new_raw(kind));
    }

    unsafe fn motion_frame(&mut self) -> f32 {
        return MotionModule::frame(self);
    }

    unsafe fn is_in_hitlag(&mut self) -> bool {
        let hitlag_frame = WorkModule::get_int(
            self,
            *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME,
        );
        if hitlag_frame > 0 {
            return true;
        }
        return false;
    }

    unsafe fn change_status_req(&mut self, kind: i32, repeat: bool) -> i32 {
        return StatusModule::change_status_request_from_script(self, kind, repeat) as i32;
    }

    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }

    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }

    unsafe fn get_num_used_jumps(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }

    unsafe fn get_jump_count_max(&mut self) -> i32 {
        return WorkModule::get_int(self, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    }

    unsafe fn get_int(&mut self, what: i32) -> i32 {
        WorkModule::get_int(self, what)
    }

    unsafe fn get_float(&mut self, what: i32) -> f32 {
        WorkModule::get_float(self, what)
    }

    unsafe fn get_int64(&mut self, what: i32) -> u64 {
        WorkModule::get_int64(self, what)
    }

    unsafe fn is_flag(&mut self, what: i32) -> bool {
        WorkModule::is_flag(self, what)
    }

    unsafe fn set_int(&mut self, value: i32, what: i32) {
        WorkModule::set_int(self, value, what)
    }

    unsafe fn set_float(&mut self, value: f32, what: i32) {
        WorkModule::set_float(self, value, what)
    }

    unsafe fn set_int64(&mut self, value: i64, what: i32) {
        WorkModule::set_int64(self, value, what)
    }

    unsafe fn on_flag(&mut self, what: i32) {
        WorkModule::on_flag(self, what)
    }

    unsafe fn off_flag(&mut self, what: i32) {
        WorkModule::off_flag(self, what)
    }

    unsafe fn get_param_int(&mut self, obj: &str, field: &str) -> i32 {
        WorkModule::get_param_int(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_float(&mut self, obj: &str, field: &str) -> f32 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_float(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn get_param_int64(&mut self, obj: &str, field: &str) -> u64 {
        let obj = obj.into();
        let field = field.into();
        WorkModule::get_param_int64(self, Hash40::new(obj).hash, Hash40::new(field).hash)
    }

    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f) {
        ModelModule::set_joint_rotate(
            self,
            Hash40::new(&bone_name),
            &rotation,
            MotionNodeRotateCompose {
                _address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8,
            },
            MotionNodeRotateOrder {
                _address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8,
            },
        )
    }

    /// gets the FighterKineticEnergyMotion object
    unsafe fn get_motion_energy(&mut self) -> &mut FighterKineticEnergyMotion {
        std::mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(
            self,
            *FIGHTER_KINETIC_ENERGY_ID_MOTION,
        ))
    }

    /// gets the FighterKineticEnergyController object
    unsafe fn get_controller_energy(&mut self) -> &mut FighterKineticEnergyController {
        std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(
            KineticModule::get_energy(self, *FIGHTER_KINETIC_ENERGY_ID_CONTROL),
        )
    }

    unsafe fn handle_waveland(&mut self, require_airdodge: bool, change_status: bool) -> bool {
        // MotionModule::frame(self) > 5.0 && !WorkModule::is_flag(self, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU);
        if require_airdodge
            && (!self.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
            ]) || (MotionModule::frame(self) > 5.0
                && !WorkModule::is_flag(self, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU)))
        {
            return false;
        }

        // must check this because it is for allowing the player to screw up a perfect WD and be punished with a non-perfect WD (otherwise they'd have like, 8 frames for perfect WD lol)
        if !crate::VarModule::is_flag(
            self.object(),
            crate::consts::vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET,
        ) {
            return false;
        }

        if self.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
            return false;
        }

        // ecb is top, bottom, left, right
        let shift = if self.is_situation(*SITUATION_KIND_AIR)
            && self.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR)
                <= crate::ParamModule::get_int(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_shift_air_trans_frame",
                ) {
            let group = crate::ParamModule::get_int(
                self.object(),
                crate::ParamType::Shared,
                "ecb_group_shift",
            );
            let shift = match group {
                0 => crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_group_shift_amount.small",
                ),
                1 => crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_group_shift_amount.medium",
                ),
                2 => crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_group_shift_amount.large",
                ),
                3 => crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_group_shift_amount.x_large",
                ),
                4 => crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_group_shift_amount.xx_large",
                ),
                _ => panic!(
                    "malformed parammodule file! unknown group number for ecb shift: {}",
                    group
                ),
            };
            shift
                + crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "ecb_shift_for_waveland",
                )
        } else {
            0.0
        };

        let ecb_bottom = *GroundModule::get_rhombus(self, true).add(1);
        let line_bottom = Vector2f::new(
            ecb_bottom.x,
            shift + ecb_bottom.y
                - crate::ParamModule::get_float(
                    self.object(),
                    crate::ParamType::Common,
                    "waveland_distance_threshold",
                ),
        );
        let mut out_pos = Vector2f::zero();
        let result = GroundModule::line_segment_check(
            self,
            &Vector2f::new(ecb_bottom.x, shift + ecb_bottom.y),
            &line_bottom,
            &Vector2f::zero(),
            &mut out_pos,
            true,
        );
        if result != 0 {
            // pretty sure it returns a pointer, at least it defo returns a non-0 value if success
            let pos = PostureModule::pos(self);
            PostureModule::set_pos(self, &Vector3f::new((*pos).x, out_pos.y + 0.01, (*pos).z));
            GroundModule::attach_ground(self, true);
            true
        } else {
            false
        }
    }

    /// gets the current status kind for the fighter
    unsafe fn status(&mut self) -> i32 {
        return StatusModule::status_kind(self);
    }

    unsafe fn shift_ecb_on_landing(&mut self) {
        if self.is_situation(*SITUATION_KIND_GROUND) {
            if !self.is_prev_situation(*SITUATION_KIND_GROUND) {
                // shift ECB back to normal offset
                let mut fighter_pos = Vector3f {
                    x: PostureModule::pos_x(self),
                    y: PostureModule::pos_y(self),
                    z: PostureModule::pos_z(self),
                };
                fighter_pos.y += crate::VarModule::get_float(
                    self.object(),
                    crate::consts::vars::common::instance::ECB_Y_OFFSETS,
                );
                PostureModule::set_pos(self, &fighter_pos);
            }
        }
    }
}

pub trait LuaUtil {
    // kinetic
    unsafe fn get_speed_x(&mut self, kinetic_id: i32) -> f32;
    unsafe fn get_speed_y(&mut self, kinetic_id: i32) -> f32;
    unsafe fn set_speed(&mut self, speed: Vector2f, kinetic_id: i32);
}

impl LuaUtil for L2CAgentBase {
    unsafe fn get_speed_x(&mut self, kinetic_id: i32) -> f32 {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id);
        app::sv_kinetic_energy::get_speed_x(self.lua_state_agent)
    }

    unsafe fn get_speed_y(&mut self, kinetic_id: i32) -> f32 {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id);
        app::sv_kinetic_energy::get_speed_y(self.lua_state_agent)
    }

    unsafe fn set_speed(&mut self, speed: Vector2f, kinetic_id: i32) {
        self.clear_lua_stack();
        smash_script::lua_args!(self, kinetic_id, speed.x, speed.y);
        app::sv_kinetic_energy::set_speed(self.lua_state_agent);
    }
}

pub trait GetObjects {
    unsafe fn boma(&mut self) -> &'static mut BattleObjectModuleAccessor {
        Self::get_boma(self)
    }

    unsafe fn object(&mut self) -> &'static mut BattleObject {
        Self::get_object(self)
    }

    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor;
    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject;
}

impl GetObjects for smash::lib::L2CAgent {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(this.battle_object)
    }
}

impl GetObjects for BattleObject {
    unsafe fn get_boma(this: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(this.module_accessor)
    }

    unsafe fn get_object(_: &mut Self) -> &'static mut BattleObject {
        panic!("Gannot call GetObjects::get_object on BattleObject!")
    }
}

impl GetObjects for BattleObjectModuleAccessor {
    unsafe fn get_boma(_: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        panic!("Gannot call GetObjects::get_boma on BattleObjectModuleAccessor!")
    }

    unsafe fn get_object(this: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(crate::util::get_battle_object_from_id(
            this.battle_object_id,
        ))
    }
}
