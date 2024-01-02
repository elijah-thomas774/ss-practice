#![allow(non_snake_case)]
use super::file_manager::FileManager;
use core::ffi::{c_ushort, c_void};

#[repr(C)]
pub struct DungeonflagManager {
    should_commit: bool,
    flagindex:     c_ushort,
}
#[repr(C)]
pub struct StoryflagManager {
    tobefilled: u32,
}
#[repr(C)]
pub struct SceneflagManager {
    tobefilled: u32,
}
#[repr(C)]
pub struct ItemflagManager {
    tobefilled: u32,
}

extern "C" {
    fn FlagManager__setFlagTo1(mgr: *mut c_void, flag: u16);
    fn FlagManager__getFlagOrCounter(mgr: *mut c_void, flag: u16) -> u16;
    fn FlagManager__setFlagOrCounter(mgr: *mut c_void, flag: u16, value: u16);

    fn setStoryflagToValue(flag: u16, value: u16);
    fn getKeyPieceCount() -> u16;
    static STORYFLAG_MANAGER: *mut StoryflagManager;
    static SCENEFLAG_MANAGER: *mut SceneflagManager;
    static ITEMFLAG_MANAGER: *mut ItemflagManager;
    static DUNGEONFLAG_MANAGER: *mut DungeonflagManager;
    static mut STATIC_STORYFLAGS: [c_ushort; 0x80];
    static mut STATIC_ITEMFLAGS: [c_ushort; 0x40];
    static mut STATIC_DUNGEON_FLAGS: [c_ushort; 8usize];
    fn SceneflagManager__setFlagGlobal(mgr: *mut SceneflagManager, scene_index: u16, flag: u16);
    fn SceneflagManager__unsetFlagGlobal(mgr: *mut SceneflagManager, scene_index: u16, flag: u16);
    fn SceneflagManager__checkFlagGlobal(
        mgr: *mut SceneflagManager,
        scene_index: u16,
        flag: u16,
    ) -> bool;
    fn StoryflagManager__doCommit(mgr: *mut StoryflagManager);
    fn ItemflagManager__doCommit(mgr: *mut ItemflagManager);
    fn checkStoryflagIsSet(p: *const StoryflagManager, flag: u16) -> bool;
    fn checkItemFlag(flag: u16) -> bool;

}

impl StoryflagManager {
    pub fn check(flag: u16) -> bool {
        unsafe { checkStoryflagIsSet(core::ptr::null(), flag) }
    }
    pub fn get_value(flag: u16) -> u16 {
        unsafe { FlagManager__getFlagOrCounter(STORYFLAG_MANAGER as _, flag) }
    }
    pub fn set_to_value(flag: u16, value: u16) {
        unsafe { FlagManager__setFlagOrCounter(STORYFLAG_MANAGER as _, flag, value) };
    }
    #[no_mangle]
    pub fn storyflag_set_to_1(flag: u16) {
        unsafe { FlagManager__setFlagTo1(STORYFLAG_MANAGER as _, flag) };
    }
}

impl ItemflagManager {
    pub fn check(flag: u16) -> bool {
        unsafe { checkItemFlag(flag) }
    }

    pub fn set_to_value(flag: u16, value: u16) {
        unsafe { FlagManager__setFlagOrCounter(ITEMFLAG_MANAGER as _, flag, value) };
    }
}

impl SceneflagManager {
    pub fn check_global(scn_idx: u16, flag: u16) -> bool {
        unsafe { SceneflagManager__checkFlagGlobal(SCENEFLAG_MANAGER, scn_idx, flag) }
    }
    pub fn set_global(scn_idx: u16, flag: u16) {
        unsafe { SceneflagManager__setFlagGlobal(SCENEFLAG_MANAGER, scn_idx, flag) };
    }
    pub fn unset_global(scn_idx: u16, flag: u16) {
        unsafe { SceneflagManager__unsetFlagGlobal(SCENEFLAG_MANAGER, scn_idx, flag) };
    }
}

impl DungeonflagManager {
    /// returns the pointer to the static dungeonflags, those for the current
    /// sceneflagindex
    pub fn get_local() -> *mut [u16; 8] {
        unsafe { &mut STATIC_DUNGEON_FLAGS }
    }
    pub fn get_global(scn_idx: u16) -> *mut [u16; 8] {
        unsafe {
            (*FileManager::GetDungeonFlags())
                .as_mut_ptr()
                .add(scn_idx as usize)
        }
    }
    pub fn get_global_key_count(scn_idx: u16) -> u16 {
        unsafe { (*Self::get_global(scn_idx))[1] & 0xF }
    }
}
