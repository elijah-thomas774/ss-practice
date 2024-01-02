#![allow(non_snake_case)]
use core::ffi::c_void;

#[repr(C)]
pub struct Reloader {
    _0:                        [u8; 0x290],
    initial_speed:             f32,
    stamina_amount:            u32,
    item_to_use_on_reload:     u8,
    beedle_shop_spawn_state:   u8,
    spawn_state:               i16, // actionIndex
    last_area_type:            u32,
    type_0_pos_flag:           u8,
    unk:                       u8,
    save_prompt_flag:          u8,
    prevent_save_respawn_info: bool,
}

extern "C" {
    static mut RELOADER_PTR: *mut Reloader;
    fn RoomManager__getRoomByIndex(room_mgr: *mut c_void, room_number: u32);
    fn Reloader__setReloadTrigger(reloader: *mut Reloader, trigger: u8);
    fn actuallyTriggerEntrance(
        stage_name: *const u8,
        room: u8,
        layer: u8,
        entrance: u8,
        forced_night: u8,
        forced_trial: u8,
        transition_type: u8,
        transition_fade_frames: u8,
        param_9: u8,
    );
}

impl Reloader {
    pub fn SetReloadTrigger(trigger: u8) {
        unsafe { Reloader__setReloadTrigger(RELOADER_PTR, trigger) };
    }
    pub fn TriggerEntrance(
        stage_name: *const u8,
        room: u8,
        layer: u8,
        entrance: u8,
        forced_night: u8,
        forced_trial: u8,
        transition_type: u8,
        transition_fade_frames: u8,
        param_9: u8,
    ) {
        unsafe {
            actuallyTriggerEntrance(
                stage_name,
                room,
                layer,
                entrance,
                forced_night,
                forced_trial,
                transition_type,
                transition_fade_frames,
                param_9,
            )
        };
    }
}
