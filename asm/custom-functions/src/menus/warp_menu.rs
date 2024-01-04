use crate::information::stage_info::*;
use crate::menus::main_menu::MainMenu;
use crate::menus::simple_menu::SimpleMenu;
use crate::system::button::*;
use crate::system::reloader::Reloader;

use core::ffi::CStr;
use cstr::cstr;

#[derive(Copy, Clone, PartialEq, Eq)]
enum WarpState {
    Off,
    Main,
    Stage,
    Details,
}

const STAGES: [&StageCategory; 7] = [
    &THE_SKY,
    &FARON,
    &ELDIN,
    &LANAYRU,
    &SEALED_GROUNDS,
    &DUNGEONS,
    &SILENT_REALMS,
];

pub struct WarpMenu {
    state:             WarpState,
    stage_selected:    [u8; 8],
    main_cursor:       u8,
    stage_cursor:      u8,
    detail_cursor:     u8,
    selected_room:     u8,
    selected_layer:    u8,
    selected_entrance: u8,
}

impl WarpMenu {
    fn get_stages(&mut self) -> &'static [StageInfo] {
        if STAGES.len() <= self.main_cursor as usize {
            self.main_cursor = 0;
        }
        STAGES[self.main_cursor as usize].stages
    }

    fn get_stage(&mut self) -> &'static StageInfo {
        let stages = self.get_stages();
        if stages.len() <= self.stage_cursor as usize {
            self.stage_cursor = 0;
        }
        &stages[self.stage_cursor as usize]
    }

    fn get_rooms(&mut self) -> &'static [u8] {
        self.get_stage().rooms
    }

    fn get_layers(&mut self) -> &'static [u8] {
        self.get_stage().layers
    }

    fn get_room(&mut self) -> u8 {
        let rooms = self.get_rooms();
        if rooms.len() <= self.selected_room as usize {
            self.selected_room = 0;
        }
        rooms[self.selected_room as usize]
    }

    fn get_layer(&mut self) -> u8 {
        let layers = self.get_layers();
        if layers.len() <= self.selected_layer as usize {
            self.selected_layer = 0;
        }
        layers[self.selected_layer as usize]
    }

    fn get_entrance(&self) -> u8 {
        self.selected_entrance
    }

    fn warp(&mut self) {
        let stage_name = self.get_stage().name;
        for n in 0..8 {
            self.stage_selected[n] = if n < stage_name.len() {
                stage_name.as_bytes()[n] as u8
            } else {
                0
            };
        }
        let room = self.get_room();
        let layer = self.get_layer();
        let entrance = self.get_entrance();
        let forced_night: u8 = match self.main_cursor {
            0 => {
                if layer % 2 == 0 {
                    0
                } else {
                    1
                }
            },
            _ => 0,
        };
        let forced_trial: u8 = if self.stage_selected[0] == b'S' { 1 } else { 0 };
        let transition_type = 0;
        unsafe {
            Reloader::TriggerEntrance(
                self.stage_selected.as_ptr(),
                room,
                layer,
                entrance,
                forced_night,
                forced_trial,
                transition_type,
                0xF,  // transition_fade_frames:  u8
                0xFF, // param_9: u8
            );
            Reloader::SetReloadTrigger(5);
        }
    }

    fn change_room(&mut self, num: i8) {
        let num_rooms = self.get_rooms().len();
        self.selected_room =
            (self.selected_room as i8 + num_rooms as i8 + num) as u8 % num_rooms as u8;
    }
    fn change_layer(&mut self, num: i8) {
        let num_layers = self.get_layers().len();
        self.selected_layer =
            (self.selected_layer as i8 + num_layers as i8 + num) as u8 % num_layers as u8;
    }
    fn change_entrance(&mut self, num: i8) {
        self.selected_entrance = (self.selected_entrance as i8 + num) as u8;
    }
}

#[link_section = "data"]
#[no_mangle]
static mut WARP_MENU: WarpMenu = WarpMenu {
    state:             WarpState::Off,
    stage_selected:    [0u8; 8],
    main_cursor:       0,
    stage_cursor:      0,
    detail_cursor:     0,
    selected_room:     0,
    selected_layer:    0,
    selected_entrance: 0,
};

impl WarpMenu {
    fn _input(&mut self) {
        let b_pressed = is_pressed(B);
        let a_pressed = is_pressed(A);
        let up_pressed = is_pressed(DPAD_UP);
        let down_pressed = is_pressed(DPAD_DOWN);
        let right_pressed = is_pressed(DPAD_RIGHT);
        let left_pressed = is_pressed(DPAD_LEFT);

        let mut next_state = self.state;

        match next_state {
            WarpState::Off => {},
            WarpState::Main => {
                if b_pressed {
                    next_state = WarpState::Off;
                } else if a_pressed {
                    next_state = WarpState::Stage;
                }
            },
            WarpState::Stage => {
                if b_pressed {
                    next_state = WarpState::Main;
                } else if a_pressed {
                    next_state = WarpState::Details;
                }
            },
            WarpState::Details => {
                if b_pressed {
                    next_state = WarpState::Stage;
                } else if a_pressed {
                    self.warp();
                    next_state = WarpState::Off;
                    MainMenu::disable();
                } else if right_pressed || left_pressed {
                    match self.detail_cursor {
                        0 => self.change_room(if right_pressed { 1 } else { -1 }),
                        1 => self.change_layer(if right_pressed { 1 } else { -1 }),
                        2 => self.change_entrance(if right_pressed { 1 } else { -1 }),
                        _ => {},
                    }
                }
            },
        }
        self.state = next_state;
    }

    fn _display(&mut self) {
        match self.state {
            WarpState::Off => {},
            WarpState::Main => {
                let mut menu =
                    SimpleMenu::<{ STAGES.len() }, 25>::new(10f32, 10f32, 10, "Warp Menu");
                for stage in STAGES {
                    menu.add_entry(stage.name);
                }
                self.main_cursor = menu.move_cursor(self.main_cursor.into()) as u8;
                menu.draw();
            },
            WarpState::Stage => {
                let stage_ref = STAGES[self.main_cursor as usize];
                let mut menu = SimpleMenu::<30, 25>::new(10f32, 10f32, 10, stage_ref.name);
                for stage in stage_ref.stages {
                    menu.add_entry(stage.name);
                }
                self.stage_cursor = menu.move_cursor(self.stage_cursor.into()) as u8;
                menu.draw();
            },
            WarpState::Details => {
                let mut detail_menu =
                    SimpleMenu::<5, 25>::new(10f32, 10f32, 10, self.get_stage().name);
                let (room, layer, entrance) =
                    (self.get_room(), self.get_layer(), self.get_entrance());
                detail_menu.add_entry_args(format_args!("Room: {room}"));
                detail_menu.add_entry_args(format_args!("Layer: {layer}"));
                detail_menu.add_entry_args(format_args!("Entrance: {entrance}"));
                self.detail_cursor = detail_menu.move_cursor(self.detail_cursor.into()) as u8;
                detail_menu.draw();
            },
        }
    }
    pub fn enable() {
        unsafe { WARP_MENU.state = WarpState::Main };
    }
    pub fn input() -> bool {
        unsafe {
            WARP_MENU._input();
            WARP_MENU.state == WarpState::Off
        }
    }

    pub fn display() {
        unsafe { WARP_MENU._display() };
    }
}
