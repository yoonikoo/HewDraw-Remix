use bitflags::bitflags;
use modular_bitfield::specifiers::*;
use smash::app::{
    self, lua_bind::*, FighterKineticEnergyController, FighterKineticEnergyMotion, *,
};
use smash::lib::{lua_const::*, *};
use smash::lua2cpp::*;
use smash::phx::*;

pub mod controls;
pub mod energy;
pub mod objects;

pub use controls::*;
pub use energy::*;
pub use objects::*;

pub trait Vec2Ext {
    fn new(x: f32, y: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

pub trait Vec4Ext {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
}

impl Vec2Ext for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vec4Ext for Vector4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AerialKind {
    Nair,
    Fair,
    Bair,
    Uair,
    Dair,
}

pub type StatusFunc = unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue;

pub struct StatusInfo {
    pub pre: Option<StatusFunc>,
    pub main: Option<StatusFunc>,
    pub end: Option<StatusFunc>,
    pub init: Option<StatusFunc>,
    pub exec: Option<StatusFunc>,
    pub exec_stop: Option<StatusFunc>,
    pub exec_post: Option<StatusFunc>,
    pub exit: Option<StatusFunc>,
    pub map_correction: Option<StatusFunc>,
    pub fix_camera: Option<StatusFunc>,
    pub fix_pos_slow: Option<StatusFunc>,
    pub check_damage: Option<StatusFunc>,
    pub check_attack: Option<StatusFunc>,
    pub on_change_lr: Option<StatusFunc>,
    pub leave_stop: Option<StatusFunc>,
    pub notify_event_gimmick: Option<StatusFunc>,
    pub calc_param: Option<StatusFunc>,
}

impl StatusInfo {
    pub fn new() -> StatusInfo {
        StatusInfo {
            pre: None,
            main: None,
            end: None,
            init: None,
            exec: None,
            exec_stop: None,
            exec_post: None,
            exit: None,
            map_correction: None,
            fix_camera: None,
            fix_pos_slow: None,
            check_damage: None,
            check_attack: None,
            on_change_lr: None,
            leave_stop: None,
            notify_event_gimmick: None,
            calc_param: None,
        }
    }

    pub fn with_pre(mut self, pre: StatusFunc) -> Self {
        self.pre = Some(pre);
        self
    }

    pub fn with_main(mut self, main: StatusFunc) -> Self {
        self.main = Some(main);
        self
    }

    pub fn with_end(mut self, end: StatusFunc) -> Self {
        self.end = Some(end);
        self
    }

    pub fn with_init(mut self, init: StatusFunc) -> Self {
        self.init = Some(init);
        self
    }

    pub fn with_exec(mut self, exec: StatusFunc) -> Self {
        self.exec = Some(exec);
        self
    }

    pub fn with_exec_stop(mut self, exec_stop: StatusFunc) -> Self {
        self.exec_stop = Some(exec_stop);
        self
    }

    pub fn with_exec_post(mut self, exec_post: StatusFunc) -> Self {
        self.exec_post = Some(exec_post);
        self
    }

    pub fn with_exit(mut self, exit: StatusFunc) -> Self {
        self.exit = Some(exit);
        self
    }

    pub fn with_map_correction(mut self, map_correction: StatusFunc) -> Self {
        self.map_correction = Some(map_correction);
        self
    }

    pub fn with_fix_camera(mut self, fix_camera: StatusFunc) -> Self {
        self.fix_camera = Some(fix_camera);
        self
    }

    pub fn with_fix_pos_slow(mut self, fix_pos_slow: StatusFunc) -> Self {
        self.fix_pos_slow = Some(fix_pos_slow);
        self
    }

    pub fn with_check_damage(mut self, check_damage: StatusFunc) -> Self {
        self.check_damage = Some(check_damage);
        self
    }

    pub fn with_check_attack(mut self, check_attack: StatusFunc) -> Self {
        self.check_attack = Some(check_attack);
        self
    }

    pub fn with_on_change_lr(mut self, on_change_lr: StatusFunc) -> Self {
        self.on_change_lr = Some(on_change_lr);
        self
    }

    pub fn with_leave_stop(mut self, leave_stop: StatusFunc) -> Self {
        self.leave_stop = Some(leave_stop);
        self
    }

    pub fn with_notify_event_gimmick(mut self, notify_event_gimmick: StatusFunc) -> Self {
        self.notify_event_gimmick = Some(notify_event_gimmick);
        self
    }

    pub fn with_calc_param(mut self, calc_param: StatusFunc) -> Self {
        self.calc_param = Some(calc_param);
        self
    }
}

pub fn is_hdr_available() -> bool {
    let mut symbol = 0usize;
    unsafe {
        skyline::nn::ro::LookupSymbol(&mut symbol, "hdr_is_available\0".as_ptr());
    }
    symbol != 0
}
