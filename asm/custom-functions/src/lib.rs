#![no_std]
#![feature(split_array)]
#![allow(unused)]

use core::{
    ffi::{c_char, c_double, c_ushort, c_void},
    ptr, slice,
};

use cstr::cstr;
use wchar::wchz;

mod information;
mod live_info;
mod menus;
mod message;
mod system;
mod utils;

use live_info::{link_pos_viewer, main::LiveInfo};
use menus::main_menu::MainMenu;
use message::{text_manager_set_num_args, text_manager_set_string_arg, FlowElement};
use system::button::*;
use system::gx::*;
use system::text_print::write_to_screen;

#[repr(C)]
struct SpawnStruct {
    name:                   [u8; 32],
    transition_fade_frames: u16,
    room:                   u8,
    layer:                  u8,
    entrance:               u8,
    night:                  u8,
    trial:                  u8,
    transition_type:        u8,
    field8_0x28:            u8,
    field9_0x29:            u8,
    field10_0x2a:           u8,
    field11_0x2b:           u8,
}

#[repr(C)]
struct ActorEventFlowMgr {
    vtable:                     u32,
    msbf_info:                  u32,
    current_flow_index:         u32,
    unk1:                       u32,
    unk2:                       u32,
    unk3:                       u32,
    result_from_previous_check: u32,
    current_text_label_name:    [u8; 32],
    unk4:                       u32,
    unk5:                       u32,
    unk6:                       u32,
    next_flow_delay_timer:      u32,
    another_flow_element:       u128,
    unk7:                       u32,
    unk8:                       u32,
}

#[repr(C)]
struct AcOBird {
    pad:   [u8; 0x144],
    speed: f32,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SpecialMinigameState {
    State0,
    BambooCutting,
    FunFunIsland,
    ThrillDigger,
    PumpkinCarry,
    InsectCaptureGame,
    PumpkinClayShooting,
    RollercoasterMinigame,
    TrialTimeAttack,
    BossRush,
    HouseCleaning,
    SpiralChargeTutorial,
    HarpPlaying,
    StateNone = -1,
}

impl SpecialMinigameState {
    pub fn get() -> Self {
        unsafe { SPECIAL_MINIGAME_STATE }
    }

    pub fn is_current(self) -> bool {
        Self::get() == self
    }
}

#[repr(C)]
struct StartInfo {
    stage:        [u8; 8],
    room:         u8,
    layer:        u8,
    entrance:     u8,
    forced_night: u8,
}

#[repr(C)]
struct ActorLink {
    base_base:      [u8; 0x60 - 0x00],
    vtable:         u32,
    obj_base_pad0:  [u8; 0x5C],
    pos_x:          f32,
    pos_y:          f32,
    pos_z:          f32,
    obj_base_pad:   [u8; 0x330 - (0x64 + 0x5C + 0xC)],
    pad01:          [u8; 0x4498 - 0x330],
    stamina_amount: u32,
    // More after
}

extern "C" {
    static mut GAME_FRAME: u32;
    static mut SPAWN_SLAVE: SpawnStruct;
    static LINK_PTR: *mut ActorLink;
    fn increaseCounter(counterId: u16, count: u16);
    fn AcItem__setFlagForItem(itemflag: u16);
    fn getModelDataFromOarc(oarc_mgr: *const c_void, oarc_str: *const c_char) -> *const c_void;
    static INPUT_BUFFER: u32;
    fn findActorByActorType(actor_type: i32, start_actor: *const c_void) -> *mut c_void;
    fn checkXZDistanceFromLink(actor: *const c_void, distance: f32) -> bool;
    static mut SPECIAL_MINIGAME_STATE: SpecialMinigameState;
    static mut ITEM_GET_BOTTLE_POUCH_SLOT: u32;
    static mut NUMBER_OF_ITEMS: u32;
    fn AcItem__setupItemParams(
        item_id: u16,
        subtype: u32,
        unk1: u32,
        sceneflag: u32,
        unk2: u32,
        unk3: u32,
    ) -> u32;
    fn AcItem__spawnItem(
        room: u32,
        item_params: u32,
        pos: u32,   // actually Vec3f
        rot: u32,   // actually Vec3s
        scale: u32, // actually Vec3f
        params2: u32,
        unk: u32,
    ) -> *mut c_void;

}

fn simple_rng(rng: &mut u32) -> u32 {
    *rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
    *rng
}

// A Common Place where Custom code can be injected to run once per frame
// Returns whether or not to stop (1 == continue)
#[no_mangle]
fn custom_main_additions() -> u32 {
    // Example menu
    if MainMenu::display() {
        return 0;
    }

    LiveInfo::display();

    return 1;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
