use super::*;

#[repr(C)]
pub struct KineticEnergyVTable {
    pub destructor: extern "C" fn(&mut KineticEnergy),
    pub deleter: extern "C" fn(*mut KineticEnergy),
    pub unk: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub update: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_speed: extern "C" fn(&mut KineticEnergy) -> *mut PaddedVec2,
    pub initialize: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_some_flag: extern "C" fn(&mut KineticEnergy) -> bool,
    pub set_some_flag: extern "C" fn(&mut KineticEnergy, bool),
    pub setup_energy:
        extern "C" fn(&mut KineticEnergy, u32, &Vector3f, u64, &mut BattleObjectModuleAccessor),
    pub clear_energy: extern "C" fn(&mut KineticEnergy),
    pub unk2: extern "C" fn(&mut KineticEnergy),
    pub set_speed: extern "C" fn(&mut KineticEnergy, &Vector2f),
    pub mul_accel: extern "C" fn(&mut KineticEnergy, &Vector2f),
    // ...
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PaddedVec2 {
    pub x: f32,
    pub y: f32,
    pub padding: u64,
}

impl PaddedVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, padding: 0 }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            padding: 0,
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[repr(C)]
pub struct KineticEnergy {
    pub vtable: &'static KineticEnergyVTable,
    pub _x8: u64, // probably padding
    pub speed: PaddedVec2,
    pub rot_speed: PaddedVec2,
    pub enable: bool,
    pub unk2: [u8; 0xF], // probably padding
    pub accel: PaddedVec2,
    pub speed_max: PaddedVec2,
    pub speed_brake: PaddedVec2,
    pub speed_limit: PaddedVec2,
    pub _x80: u8,
    pub consider_ground_friction: bool,
    pub active_flag: bool, // no clue?
    pub _x83: u8,
    pub energy_reset_type: u32,
}

#[repr(simd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(simd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(simd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl KineticEnergy {
    pub fn adjust_speed_for_ground_normal(
        speed: &PaddedVec2,
        boma: &mut BattleObjectModuleAccessor,
    ) -> PaddedVec2 {
        #[skyline::from_offset(0x47b4d0)]
        extern "C" fn adjust_speed_for_ground_normal_internal(
            speed: Vec2,
            boma: &mut BattleObjectModuleAccessor,
        ) -> Vec2;

        unsafe {
            let result = adjust_speed_for_ground_normal_internal(
                Vec2 {
                    x: speed.x,
                    y: speed.y,
                },
                boma,
            );
            PaddedVec2::new(result.x, result.y)
        }
    }

    pub fn process(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            #[skyline::from_offset(0x47bf70)]
            extern "C" fn process_energy(
                energy: &mut KineticEnergy,
                boma: &mut BattleObjectModuleAccessor,
            );

            process_energy(self, boma)
        }
    }

    pub fn update(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe { (self.vtable.update)(self, boma) }
    }

    pub fn get_speed<'a>(&'a mut self) -> &'a mut PaddedVec2 {
        unsafe { std::mem::transmute((self.vtable.get_speed)(self)) }
    }

    pub fn initialize(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe { (self.vtable.initialize)(self, boma) }
    }

    pub fn get_some_flag(&mut self) -> bool {
        unsafe { (self.vtable.get_some_flag)(self) }
    }

    pub fn set_some_flag(&mut self, flag: bool) {
        unsafe { (self.vtable.set_some_flag)(self, flag) }
    }

    pub fn setup_energy(
        &mut self,
        reset_type: u32,
        incoming_speed: &Vector3f,
        some: u64,
        boma: &mut BattleObjectModuleAccessor,
    ) {
        unsafe { (self.vtable.setup_energy)(self, reset_type, incoming_speed, some, boma) }
    }

    pub fn clear_energy(&mut self) {
        unsafe { (self.vtable.clear_energy)(self) }
    }

    pub fn unk2(&mut self) {
        unsafe { (self.vtable.unk2)(self) }
    }

    pub fn set_speed(&mut self, speed: &Vector2f) {
        unsafe { (self.vtable.set_speed)(self, speed) }
    }

    pub fn mul_accel(&mut self, mul: &Vector2f) {
        unsafe { (self.vtable.mul_accel)(self, mul) }
    }
}

#[repr(C)]
pub struct FlyData {
    pub turn_stick_x: f32,
    pub init_speed_x_mul: f32,
    pub speed_x_mul: f32,
    pub speed_x_max_mul: f32,
    pub speed_y_table_start: *const f32,
    pub speed_y_table_end: *const f32,
    pub speed_y_table_eos: *const f32,
    pub turn_param_start: *const i32,
    pub turn_param_end: *const i32,
    pub turn_param_eos: *const i32,
    pub shoot_fly_next_frame: i32,
}

impl FlyData {
    pub fn get_from_fighter_kind(kind: i32) -> Option<&'static Self> {
        #[repr(C)]
        struct FlyDataResult {
            vtable: *const *const (),
            data: *const *const FlyData,
        }

        unsafe {
            let accessor = *((crate::singletons::FighterParamAccessor2() as *const u8)
                .add((kind as usize) * 0x38 + 0x70) as *const u64);
            let function: extern "C" fn(u64, u64) -> FlyDataResult =
                std::mem::transmute(*(*(accessor as *const *const u64)).add(0x2));
            let result = function(accessor, smash::hash40("fly_data"));
            if (*result.data).is_null() {
                return None;
            } else {
                return Some(&**result.data);
            }
        }
    }
}

use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyControllerResetType {
    FallAdjust = 0x0,
    FallAdjustNoCap,
    StopCeil,
    WallJump,
    FlyAdjust,
    Dash,
    ShootDash,
    ShootBackDash,
    TurnRun,
    RevolveSlashAir,
    Turn,
    Free,
    FreeTest,
    ItemLift,
    SwimRise,
    Swim,
    SwimDrown,
    MoveGround,
    MoveAir,
    TurnNoStop,
    TurnNoStopAir,
    Ladder,
    DashBack,
}

#[repr(C)]
pub struct FighterKineticEnergyControl {
    parent: super::energy::KineticEnergy,
    pub lr: f32,
    pub accel_mul_x: f32,
    pub accel_add_x: f32,
    pub accel_mul_y: f32,
    pub accel_add_y: f32,
    pub _x9c: f32,
    pub _xa0: f32,
    pub unk: [u8; 4],
}

impl Deref for FighterKineticEnergyControl {
    type Target = super::energy::KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}
