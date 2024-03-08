use core::ffi::c_void;

#[repr(C)]
pub enum ItemID {
    EmptyWardrobe              = 000,
    SmallKey                   = 001,
    GreenRupee                 = 002,
    BlueRupee                  = 003,
    RedRupee                   = 004,
    Triforce                   = 005,
    Heart                      = 006,
    Arrow1                     = 007,
    Arrows10                   = 008,
    GoddessWhiteSword          = 009,
    PracticeSword              = 010,
    GoddessSword               = 011,
    GoddessLongsword           = 012,
    MasterSword                = 013,
    TrueMasterSword            = 014,
    Sailcloth                  = 015,
    GoddesssHarp               = 016,
    SpiritVessel               = 017,
    Item18                     = 018,
    Bow                        = 019,
    Clawshots                  = 020,
    BirdStatuette              = 021,
    Item22                     = 022,
    Item23                     = 023,
    Item24                     = 024,
    BlessedIdol                = 025,
    MysteriousCrystals         = 026,
    SquidCarving               = 027,
    KeyPiece                   = 028,
    GoldenCarving              = 029,
    DragonSculpture            = 030,
    AncientCircuit             = 031,
    SilverRupee                = 032,
    GoldRupee                  = 033,
    Rupoor                     = 034,
    Item35                     = 035,
    GlitteringSpores           = 036,
    Item37                     = 037,
    Item38                     = 038,
    Item39                     = 039,
    Bombs5                     = 040,
    Bombs10                    = 041,
    StaminaFruit               = 042,
    TearOfFarore               = 043,
    TearOfDin                  = 044,
    TearOfNayru                = 045,
    SacredTear                 = 046,
    LightFruit                 = 047,
    GratitudeCrystal1          = 048,
    GustBellows                = 049,
    DungeonMap                 = 050,
    Item51                     = 051,
    Slingshot                  = 052,
    Beetle                     = 053,
    Water                      = 054,
    MushroomSpores             = 055,
    DiggingMitts               = 056,
    DekuSeeds5                 = 057,
    Item58                     = 058,
    Item59                     = 059,
    Item60                     = 060,
    Item61                     = 061,
    Item62                     = 062,
    Item63                     = 063,
    Item64                     = 064,
    GuardianPotion             = 065,
    GuardianPotionPlus         = 066,
    Item67                     = 067,
    WaterDragonsScale          = 068,
    Item69                     = 069,
    BugMedal                   = 070,
    BugNet                     = 071,
    Fairy                      = 072,
    Item73                     = 073,
    SacredWater                = 074,
    HookBeetle                 = 075,
    QuickBeetle                = 076,
    ToughBeetle                = 077,
    HeartPotion                = 078,
    HeartPotionPlus            = 079,
    Item80                     = 080,
    HeartPotionPlusPlus        = 081,
    Item82                     = 082,
    Item83                     = 083,
    StaminaPotion              = 084,
    StaminaPotionPlus          = 085,
    AirPotion                  = 086,
    AirPotionPlus              = 087,
    FairyBottle                = 088,
    Item89                     = 089,
    IronBow                    = 090,
    SacredBow                  = 091,
    BombBag                    = 092,
    HeartContainer             = 093,
    PieceOfHeart               = 094,
    TriforceOfCourage          = 095,
    TriforceOfPower            = 096,
    TriforceOfWisdom           = 097,
    AncientSeaChart            = 098,
    MogmaMitts                 = 099,
    HeartMedal                 = 100,
    RupeeMedal                 = 101,
    TreasureMedal              = 102,
    PotionMedal                = 103,
    CursedMedal                = 104,
    Scattershot                = 105,
    Item106                    = 106,
    SmallWallet                = 107,
    MediumWallet               = 108,
    BigWallet                  = 109,
    GiantWallet                = 110,
    TycoonWallet               = 111,
    AdventurePouch             = 112,
    EmptyPocket                = 113,
    LifeMedal                  = 114,
    Item115                    = 115,
    WoodenShield               = 116,
    BandedShield               = 117,
    BracedShield               = 118,
    IronShield                 = 119,
    ReinforcedShield           = 120,
    FortifiedShield            = 121,
    SacredShield               = 122,
    DivineShield               = 123,
    GoddessShield              = 124,
    HylianShield               = 125,
    RevitalizingPotion         = 126,
    RevitalizingPotionPlus     = 127,
    SmallSeedSatchel           = 128,
    MediumSeedSatchel          = 129,
    LargeSeedSatchel           = 130,
    SmallQuiver                = 131,
    MediumQuiver               = 132,
    LargeQuiver                = 133,
    SmallBombBag               = 134,
    MediumBombBag              = 135,
    LargeBombBag               = 136,
    Whip                       = 137,
    FireshieldEarrings         = 138,
    Item139                    = 139,
    BigBugNet                  = 140,
    FaronGrasshopper           = 141,
    WoodlandRhinoBeetle        = 142,
    DekuHornet                 = 143,
    SkyloftMantis              = 144,
    VolcanicLadybug            = 145,
    BlessedButterfly           = 146,
    LanayruAnt                 = 147,
    SandCicada                 = 148,
    GerudoDragonfly            = 149,
    EldinRoller                = 150,
    SkyStagBeetle              = 151,
    StarryFirefly              = 152,
    EmptyBottle                = 153,
    Item154                    = 154,
    Item155                    = 155,
    Item156                    = 156,
    Item157                    = 157,
    CawlinsLetter              = 158,
    BeedlesInsectCage          = 159,
    Rattle                     = 160,
    HornetLarvae               = 161,
    BirdFeather                = 162,
    Tumbleweed                 = 163,
    LizardTail                 = 164,
    EldinOre                   = 165,
    AncientFlower              = 166,
    AmberRelic                 = 167,
    DuskRelic                  = 168,
    JellyBlob                  = 169,
    MonsterClaw                = 170,
    MonsterHorn                = 171,
    OrnamentalSkull            = 172,
    EvilCrystal                = 173,
    BlueBirdFeather            = 174,
    GoldenSkull                = 175,
    GoddessPlume               = 176,
    AncientTabletEmerald       = 177,
    AncientTabletRuby          = 178,
    AncientTabletAmber         = 179,
    StoneOfTrials              = 180,
    Item181                    = 181,
    Item182                    = 182,
    Item183                    = 183,
    Item184                    = 184,
    Item185                    = 185,
    BalladOfTheGoddess         = 186,
    FaroresCourage             = 187,
    NayrusWisdom               = 188,
    DinsPower                  = 189,
    WaterDragonSong            = 190,
    FireDragonSong             = 191,
    ThunderDragonSong          = 192,
    SongOfTheHero              = 193,
    RevitalizingPotionPlusPlus = 194,
    HotPumpkinSoup             = 195,
    ColdPumpkinSoup            = 196,
    LifeTreeSeedling           = 197,
    LifeTreeFruit              = 198,
    ExtraWallet                = 199,
    Item200                    = 200,
    ItemMax                    = 999,
}

extern "C" {
    static mut ITEM_GET_BOTTLE_POUCH_SLOT: u32;
    static mut NUMBER_OF_ITEMS: u32;

    fn getKeyPieceCount() -> u16;
    fn AcItem__setFlagForItem(itemflag: u16);
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
    fn AcItem__giveItem(item_id: u16, pouch_slot: u32, number: u32);
}

pub fn get_bottle_pouch_slot() -> u32 {
    unsafe { ITEM_GET_BOTTLE_POUCH_SLOT }
}

pub fn set_bottle_pouch_slot(val: u32) {
    unsafe { ITEM_GET_BOTTLE_POUCH_SLOT = val };
}

pub fn get_number_of_items() -> u32 {
    unsafe { NUMBER_OF_ITEMS }
}

pub fn set_number_of_items(val: u32) {
    unsafe { NUMBER_OF_ITEMS = val };
}

pub fn get_key_piece_count() -> u16 {
    unsafe { getKeyPieceCount() }
}

pub fn spawn_item(
    room: u32,
    item_params: u32,
    pos: u32,   // actually Vec3f
    rot: u32,   // actually Vec3s
    scale: u32, // actually Vec3f
    params2: u32,
    unk: u32,
) -> *mut c_void {
    unsafe { AcItem__spawnItem(room, item_params, pos, rot, scale, params2, unk) }
}

pub fn setup_item_params(
    item_id: u16,
    subtype: u32,
    unk1: u32,
    sceneflag: u32,
    unk2: u32,
    unk3: u32,
) -> u32 {
    unsafe { AcItem__setupItemParams(item_id, subtype, unk1, sceneflag, unk2, unk3) }
}

pub fn set_flag_for_item(itemflag: u16) {
    unsafe { AcItem__setFlagForItem(itemflag) };
}

pub fn give_item(item_id: u16, pouch_slot: u32, number: u32) {
    unsafe { AcItem__giveItem(item_id, pouch_slot, number) };
}
