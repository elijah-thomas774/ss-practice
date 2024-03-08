struct ItemFlagDescriptor {
    pub name: &'static str,
    idx:      u8,
}

impl ItemFlagDescriptor {
    pub fn get_idx(&self) -> u8 {
        todo!()
    }
    pub fn get_mask(&self) -> u16 {
        todo!()
    }
}
const ITEM_FLAGS_DESCRIPTIONS: [ItemFlagDescriptor; 4] = [
    ItemFlagDescriptor {
        name: "Clawshots",
        idx:  0,
    },
    ItemFlagDescriptor {
        name: "Bow",
        idx:  0,
    },
    ItemFlagDescriptor {
        name: "Master Sword",
        idx:  0,
    },
    ItemFlagDescriptor {
        name: "Goddess Longsword",
        idx:  0,
    },
];
