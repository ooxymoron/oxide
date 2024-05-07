use std::{
    alloc::{alloc, Layout},
    mem::MaybeUninit,
    ops::Sub,
};

use crate::{cfn, interface, math::vector3::Vector3, vmt_call};

use super::{
    entity::{player::Player, Entity},
    model_info::HitboxId,
    networkable::ClassId,
    WithVmt,
};

pub type EngineTrace = WithVmt<VMTEngineTrace>;

#[repr(C, align(16))]
#[derive(Debug, Clone)]
pub struct VectorAligned {
    x: f32,
    y: f32,
    z: f32,
    _pad: i32,
}
impl VectorAligned {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        VectorAligned { x, y, z, _pad: 0 }
    }
}
impl Default for VectorAligned {
    fn default() -> Self {
        VectorAligned::new(0f32, 0f32, 0f32)
    }
}

impl Sub for VectorAligned {
    type Output = VectorAligned;

    fn sub(self, rhs: Self) -> Self::Output {
        VectorAligned::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ray {
    pub start: VectorAligned,
    pub delta: VectorAligned,
    pub start_offset: VectorAligned,
    pub extents: VectorAligned,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    fn new(start: Vector3, end: Vector3) -> Self {
        let delta = end - start.clone();
        Ray {
            start: start.clone().into(),
            delta: delta.clone().into(),
            start_offset: VectorAligned::default(),
            extents: VectorAligned::default(),
            is_ray: true,
            is_swept: delta.len() != 0f32,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTTraceFilter {
    should_hit_entity: cfn!(bool, *const TraceFilter, *const Entity, i32),
    get_trace_type: cfn!(TraceType, *const TraceFilter),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TraceFilter<'a> {
    vmt: *const VMTTraceFilter,
    p_local: &'a Player,
}

pub enum TraceType {
    Everything = 0,
    WorldOnly,
    EntitiesOnly,
    EverythingFilterProps,
}

extern "C" fn should_hit_entity(
    trace_filter: *const TraceFilter,
    ent: *const Entity,
    _: i32,
) -> bool {
    if ent as *const _ == unsafe { trace_filter.read().p_local } as *const Player as *const _ {
        return false;
    }
    let networkable = unsafe { (*ent).as_networkable() };
    let class = networkable.get_client_class();
    match class.class_id {
        ClassId::CFuncRespawnRoomVisualizer => return false,
        _ => {}
    }
    return true;
}

extern "C" fn get_trace_type(_: *const TraceFilter) -> TraceType {
    TraceType::Everything
}

impl TraceFilter<'_> {
    pub fn new(p_local: &Player) -> TraceFilter {
        unsafe {
            let alloc = alloc(Layout::new::<VMTTraceFilter>());
            let ptr = alloc as *mut VMTTraceFilter;
            *ptr = VMTTraceFilter {
                should_hit_entity,
                get_trace_type,
            };
            TraceFilter { vmt: ptr, p_local }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Plane {
    normal: Vector3,
    dist: f32,
    r#type: u8,
    signbits: u8,
    pad: [u8; 2],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Surface {
    name: *const u8,
    surface_props: i16,
    flags: u16,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Trace {
    pub startpos: Vector3,
    pub endpos: Vector3,
    pub plane: Plane,
    pub fraction: f32,
    pub contents: i32,
    pub disp_flags: u16,
    pub allsolid: bool,
    pub startsolid: bool,
    pub fraction_left_solid: f32,
    pub surface: Surface,
    pub hit_group: i32,
    pub physics_bone: i16,
    pub entity: *const Entity,
    pub hitbox_id: HitboxId,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTEngineTrace {
    _pad1: [isize; 4],
    pub trace_ray: cfn!(i32, *const EngineTrace, &Ray, u32, &TraceFilter, &mut Trace),
}

pub fn trace(start: Vector3, end: Vector3, mask: u32) -> Trace {
    let p_local = Player::get_local().unwrap();
    let trace_engine = interface!(engine_trace);

    let ray = Ray::new(start, end);
    let filter = TraceFilter::new(p_local);
    let mut trace = unsafe { MaybeUninit::zeroed().assume_init() };

    vmt_call!(trace_engine, trace_ray, &ray, mask, &filter, &mut trace);
    trace
}

pub const CONTENTS_EMPTY: u32 = 0x0;
pub const CONTENTS_SOLID: u32 = 0x1;
pub const CONTENTS_WINDOW: u32 = 0x2;
pub const CONTENTS_AUX: u32 = 0x4;
pub const CONTENTS_GRATE: u32 = 0x8;
pub const CONTENTS_SLIME: u32 = 0x10;
pub const CONTENTS_WATER: u32 = 0x20;
pub const CONTENTS_BLOCKLOS: u32 = 0x40;
pub const CONTENTS_OPAQUE: u32 = 0x80;
pub const LAST_VISIBLE_CONTENTS: u32 = 0x80;
pub const ALL_VISIBLE_CONTENTS: u32 = LAST_VISIBLE_CONTENTS | (LAST_VISIBLE_CONTENTS - 1);
pub const CONTENTS_TESTFOGVOLUME: u32 = 0x100;
pub const CONTENTS_UNUSED: u32 = 0x200;
pub const CONTENTS_UNUSED6: u32 = 0x400;
pub const CONTENTS_TEAM1: u32 = 0x800;
pub const CONTENTS_TEAM2: u32 = 0x1000;
pub const CONTENTS_IGNORE_NODRAW_OPAQUE: u32 = 0x2000;
pub const CONTENTS_MOVEABLE: u32 = 0x4000;
pub const CONTENTS_AREAPORTAL: u32 = 0x8000;
pub const CONTENTS_PLAYERCLIP: u32 = 0x10000;
pub const CONTENTS_MONSTERCLIP: u32 = 0x20000;
pub const CONTENTS_CURRENT_0: u32 = 0x40000;
pub const CONTENTS_CURRENT_90: u32 = 0x80000;
pub const CONTENTS_CURRENT_180: u32 = 0x100000;
pub const CONTENTS_CURRENT_270: u32 = 0x200000;
pub const CONTENTS_CURRENT_UP: u32 = 0x400000;
pub const CONTENTS_CURRENT_DOWN: u32 = 0x800000;
pub const CONTENTS_ORIGIN: u32 = 0x1000000;
pub const CONTENTS_MONSTER: u32 = 0x2000000;
pub const CONTENTS_DEBRIS: u32 = 0x4000000;
pub const CONTENTS_DETAIL: u32 = 0x8000000;
pub const CONTENTS_TRANSLUCENT: u32 = 0x10000000;
pub const CONTENTS_LADDER: u32 = 0x20000000;
pub const CONTENTS_HITBOX: u32 = 0x40000000;

pub const MASK_ALL: u32 = 0xFFFFFFFF;
pub const MASK_SOLID: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTER | CONTENTS_GRATE;
pub const MASK_PLAYERSOLID: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_PLAYERCLIP
    | CONTENTS_WINDOW
    | CONTENTS_MONSTER
    | CONTENTS_GRATE;
pub const MASK_NPCSOLID: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTERCLIP
    | CONTENTS_WINDOW
    | CONTENTS_MONSTER
    | CONTENTS_GRATE;
pub const MASK_WATER: u32 = CONTENTS_WATER | CONTENTS_MOVEABLE | CONTENTS_SLIME;
pub const MASK_OPAQUE: u32 = CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_OPAQUE;
pub const MASK_OPAQUE_AND_NPCS: u32 = MASK_OPAQUE | CONTENTS_MONSTER;
pub const MASK_BLOCKLOS: u32 = CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_BLOCKLOS;
pub const MASK_BLOCKLOS_AND_NPCS: u32 = MASK_BLOCKLOS | CONTENTS_MONSTER;
pub const MASK_VISIBLE: u32 = MASK_OPAQUE | CONTENTS_IGNORE_NODRAW_OPAQUE;
pub const MASK_VISIBLE_AND_NPCS: u32 = MASK_OPAQUE_AND_NPCS | CONTENTS_IGNORE_NODRAW_OPAQUE;
pub const MASK_SHOT: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_HITBOX;
pub const MASK_SHOT_HULL: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_GRATE;
pub const MASK_SHOT_PORTAL: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTER;
pub const MASK_SOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_GRATE;
pub const MASK_PLAYERSOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_PLAYERCLIP | CONTENTS_GRATE;
pub const MASK_NPCSOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP | CONTENTS_GRATE;
pub const MASK_NPCWORLDSTATIC: u32 =
    CONTENTS_SOLID | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP | CONTENTS_GRATE;
pub const MASK_SPLITAREAPORTAL: u32 = CONTENTS_WATER | CONTENTS_SLIME;
pub const MASK_CURRENT: u32 = CONTENTS_CURRENT_0
    | CONTENTS_CURRENT_90
    | CONTENTS_CURRENT_180
    | CONTENTS_CURRENT_270
    | CONTENTS_CURRENT_UP
    | CONTENTS_CURRENT_DOWN;
pub const MASK_DEADSOLID: u32 =
    CONTENTS_SOLID | CONTENTS_PLAYERCLIP | CONTENTS_WINDOW | CONTENTS_GRATE;
