/// WoW Client Enumerations
/// These enums are ported directly from the WoW client and should NEVER be modified
/// as they represent the exact memory layout and values used by the game.

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum TypeMask {
    /// Basic object type
    Object = 0x1,
    /// Item type
    Item = 0x2,
    /// Container type (bags)
    Container = 0x4,
    /// Unit type (NPCs, creatures)
    Unit = 0x8,
    /// Player type
    Player = 0x10,
    /// GameObject type
    GameObject = 0x20,
    /// DynamicObject type
    DynamicObject = 0x40,
    /// Corpse type
    Corpse = 0x80,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum TypeId {
    Object = 0,
    Item = 1,
    Container = 2,
    Unit = 3,
    Player = 4,
    GameObject = 5,
    DynamicObject = 6,
    Corpse = 7,
}

pub const NUM_TYPEIDS: u32 = 8;

// Unit flags enum - comprehensive list from C++
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UnitFlags {
    /// set only when unit movement is controlled by server - by SPLINE/MONSTER_MOVE packets, together with UNIT_FLAG_STUNNED; only set to units controlled by client; client function CGUnit_C::IsClientControlled returns false when set for owner
    ServerControlled = 0x00000001,
    /// not attackable, set when creature starts to cast spells with SPELL_EFFECT_SPAWN and cast time, removed when spell hits caster, original name is UNIT_FLAG_SPAWNING. Rename when it will be removed from all scripts
    NonAttackable = 0x00000002,
    /// This is a legacy flag used to disable movement player's movement while controlling other units, SMSG_CLIENT_CONTROL replaces this functionality clientside now. CONFUSED and FLEEING flags have the same effect on client movement asDISABLE_MOVE_CONTROL in addition to preventing spell casts/autoattack (they all allow climbing steeper hills and emotes while moving)
    RemoveClientControl = 0x00000004,
    /// controlled by player, use _IMMUNE_TO_PC instead of _IMMUNE_TO_NPC
    PlayerControlled = 0x00000008,
    Rename = 0x00000010,
    /// don't take reagents for spells with SPELL_ATTR5_NO_REAGENT_WHILE_PREP
    Preparation = 0x00000020,
    Unk6 = 0x00000040,
    /// ?? (UNIT_FLAG_PLAYER_CONTROLLED | UNIT_FLAG_NOT_ATTACKABLE_1) is NON_PVP_ATTACKABLE
    NotAttackable1 = 0x00000080,
    /// disables combat/assistance with PlayerCharacters (PC) - see Unit::IsValidAttackTarget, Unit::IsValidAssistTarget
    ImmuneToPc = 0x00000100,
    /// disables combat/assistance with NonPlayerCharacters (NPC) - see Unit::IsValidAttackTarget, Unit::IsValidAssistTarget
    ImmuneToNpc = 0x00000200,
    /// loot animation
    Looting = 0x00000400,
    /// on player pets: whether the pet is chasing a target to attack || on other units: whether any of the unit's minions is in combat
    PetInCombat = 0x00000800,
    /// changed in 3.0.3, now UNIT_BYTES_2_OFFSET_PVP_FLAG from UNIT_FIELD_BYTES_2
    PvpEnabling = 0x00001000,
    /// silenced, 2.1.1
    Silenced = 0x00002000,
    /// TITLE Can't Swim
    CantSwim = 0x00004000,
    /// TITLE Can Swim DESCRIPTION shows swim animation in water
    CanSwim = 0x00008000,
    /// removes attackable icon, if on yourself, cannot assist self but can cast TARGET_SELF spells - added by SPELL_AURA_MOD_UNATTACKABLE
    NonAttackable2 = 0x00010000,
    /// 3.0.3 ok
    Pacified = 0x00020000,
    /// 3.0.3 ok
    Stunned = 0x00040000,
    InCombat = 0x00080000,
    /// disable casting at client side spell not allowed by taxi flight (mounted?), probably used with 0x4 flag
    OnTaxi = 0x00100000,
    /// 3.0.3, disable melee spells casting..., "Required melee weapon" added to melee spells tooltip.
    Disarmed = 0x00200000,
    Confused = 0x00400000,
    Fleeing = 0x00800000,
    /// under direct client control by a player (possess or vehicle)
    Possessed = 0x01000000,
    Uninteractible = 0x02000000,
    Skinnable = 0x04000000,
    Mount = 0x08000000,
    Unk28 = 0x10000000,
    /// Prevent automatically playing emotes from parsing chat text, for example "lol" in /say, ending message with ? or !, or using /yell
    PreventEmotesFromChatText = 0x20000000,
    Sheathe = 0x40000000,
    /// Immune to damage
    Immune = 0x80000000,
}

pub const UNIT_FLAG_DISALLOWED: u32 = UnitFlags::ServerControlled as u32 |
    UnitFlags::NonAttackable as u32 |
    UnitFlags::RemoveClientControl as u32 |
    UnitFlags::PlayerControlled as u32 |
    UnitFlags::Rename as u32 |
    UnitFlags::Preparation as u32 |
    // UnitFlags::Unk6 as u32 |
    UnitFlags::NotAttackable1 as u32 |
    UnitFlags::Looting as u32 |
    UnitFlags::PetInCombat as u32 |
    UnitFlags::PvpEnabling as u32 |
    UnitFlags::Silenced as u32 |
    UnitFlags::NonAttackable2 as u32 |
    UnitFlags::Pacified as u32 |
    UnitFlags::Stunned as u32 |
    UnitFlags::InCombat as u32 |
    UnitFlags::OnTaxi as u32 |
    UnitFlags::Disarmed as u32 |
    UnitFlags::Confused as u32 |
    UnitFlags::Fleeing as u32 |
    UnitFlags::Possessed as u32 |
    UnitFlags::Skinnable as u32 |
    UnitFlags::Mount as u32 |
    UnitFlags::Unk28 as u32 |
    UnitFlags::PreventEmotesFromChatText as u32 |
    UnitFlags::Sheathe as u32 |
    UnitFlags::Immune as u32;

pub const UNIT_FLAG_ALLOWED: u32 = 0xFFFFFFFF & !UNIT_FLAG_DISALLOWED;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UnitDynFlags {
    None = 0x0000,
    Lootable = 0x0001,
    TrackUnit = 0x0002,
    /// Lua_UnitIsTapped
    Tapped = 0x0004,
    /// Lua_UnitIsTappedByPlayer
    TappedByPlayer = 0x0008,
    SpecialInfo = 0x0010,
    Dead = 0x0020,
    ReferAFriend = 0x0040,
    /// Lua_UnitIsTappedByAllThreatList
    TappedByAllThreatList = 0x0080,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GameObjectType {
    Door = 0,
    Button = 1,
    QuestGiver = 2,
    Chest = 3,
    Binder = 4,
    Generic = 5,
    Trap = 6,
    Chair = 7,
    SpellFocus = 8,
    Text = 9,
    Goober = 10,
    Transport = 11,
    AreaDamage = 12,
    Camera = 13,
    MapObject = 14,
    MoTransport = 15,
    DuelArbiter = 16,
    FishingNode = 17,
    SummoningRitual = 18,
    Mailbox = 19,
    DoNotUse = 20,
    GuardPost = 21,
    SpellCaster = 22,
    MeetingStone = 23,
    FlagStand = 24,
    FishingHole = 25,
    FlagDrop = 26,
    MiniGame = 27,
    DoNotUse2 = 28,
    CapturePoint = 29,
    AuraGenerator = 30,
    DungeonDifficulty = 31,
    BarberChair = 32,
    DestructibleBuilding = 33,
    GuildBank = 34,
    TrapDoor = 35,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GameObjectFlags {
    None = 0x00000000,
    InUse = 0x00000001,
    Locked = 0x00000002,
    InteractCond = 0x00000004,
    Transport = 0x00000008,
    NotSelectable = 0x00000010,
    NoSpawn = 0x00000020,
    Triggered = 0x00000040,
    Damaged = 0x00000200,
    Destroyed = 0x00000400,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectFlags {
    Unit = 0x8,
}

/// Object fields - these values are used for memory access
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectFields {
    /// Size: 2, Type: LONG, Flags: PUBLIC
    Guid = 0x0000,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Type = 0x0002,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Entry = 0x0003,
    /// Size: 1, Type: FLOAT, Flags: PUBLIC
    ScaleX = 0x0004,
    /// Size: 1, Type: INT, Flags: NONE
    Padding = 0x0005,
    End = 0x0006,
}

/// GameObject fields - these values are used for memory access
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GameObjectFields {
    /// Size: 2, Type: LONG, Flags: PUBLIC
    CreatedBy = ObjectFields::End as u32 + 0x0000,
    /// Size: 1, Type: INT, Flags: PUBLIC
    DisplayId = ObjectFields::End as u32 + 0x0002,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Flags = ObjectFields::End as u32 + 0x0003,
    /// Size: 4, Type: FLOAT, Flags: PUBLIC
    ParentRotation = ObjectFields::End as u32 + 0x0004,
    /// Size: 1, Type: TWO_SHORT, Flags: DYNAMIC
    Dynamic = ObjectFields::End as u32 + 0x0008,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Faction = ObjectFields::End as u32 + 0x0009,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Level = ObjectFields::End as u32 + 0x000A,
    /// Size: 1, Type: BYTES, Flags: PUBLIC
    Bytes1 = ObjectFields::End as u32 + 0x000B,
    End = ObjectFields::End as u32 + 0x000C,
}

/// Unit field enumeration - complete matching C++ EUnitFields
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum UnitFields {
    /// Size: 2, Type: LONG, Flags: PUBLIC
    Charm = ObjectFields::End as u32 + 0x0000,
    /// Size: 2, Type: LONG, Flags: PUBLIC
    Summon = ObjectFields::End as u32 + 0x0002,
    /// Size: 2, Type: LONG, Flags: PRIVATE
    Critter = ObjectFields::End as u32 + 0x0004,
    /// Size: 2, Type: LONG, Flags: PUBLIC
    CharmedBy = ObjectFields::End as u32 + 0x0006,
    /// Size: 2, Type: LONG, Flags: PUBLIC
    SummonedBy = ObjectFields::End as u32 + 0x0008,
    /// Size: 2, Type: LONG, Flags: PUBLIC
    CreatedBy = ObjectFields::End as u32 + 0x000A,
    /// Size: 2, Type: LONG, Flags: PUBLIC
    Target = ObjectFields::End as u32 + 0x000C,
    /// Size: 2, Type: LONG, Flags: PUBLIC
    ChannelObject = ObjectFields::End as u32 + 0x000E,
    /// Size: 1, Type: INT, Flags: PUBLIC
    ChannelSpell = ObjectFields::End as u32 + 0x0010,
    /// Size: 1, Type: BYTES, Flags: PUBLIC
    Bytes0 = ObjectFields::End as u32 + 0x0011,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Health = ObjectFields::End as u32 + 0x0012,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power1 = ObjectFields::End as u32 + 0x0013,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power2 = ObjectFields::End as u32 + 0x0014,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power3 = ObjectFields::End as u32 + 0x0015,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power4 = ObjectFields::End as u32 + 0x0016,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power5 = ObjectFields::End as u32 + 0x0017,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power6 = ObjectFields::End as u32 + 0x0018,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Power7 = ObjectFields::End as u32 + 0x0019,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxHealth = ObjectFields::End as u32 + 0x001A,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower1 = ObjectFields::End as u32 + 0x001B,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower2 = ObjectFields::End as u32 + 0x001C,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower3 = ObjectFields::End as u32 + 0x001D,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower4 = ObjectFields::End as u32 + 0x001E,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower5 = ObjectFields::End as u32 + 0x001F,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower6 = ObjectFields::End as u32 + 0x0020,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MaxPower7 = ObjectFields::End as u32 + 0x0021,
    /// Size: 7, Type: FLOAT, Flags: PRIVATE, OWNER
    PowerRegenFlatModifier = ObjectFields::End as u32 + 0x0022,
    /// Size: 7, Type: FLOAT, Flags: PRIVATE, OWNER
    PowerRegenInterruptedFlatModifier = ObjectFields::End as u32 + 0x0029,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Level = ObjectFields::End as u32 + 0x0030,
    /// Size: 1, Type: INT, Flags: PUBLIC
    FactionTemplate = ObjectFields::End as u32 + 0x0031,
    /// Size: 3, Type: INT, Flags: PUBLIC
    VirtualItemSlotId = ObjectFields::End as u32 + 0x0032,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Flags = ObjectFields::End as u32 + 0x0035,
    /// Size: 1, Type: INT, Flags: PUBLIC
    Flags2 = ObjectFields::End as u32 + 0x0036,
    /// Size: 1, Type: INT, Flags: PUBLIC
    AuraState = ObjectFields::End as u32 + 0x0037,
    /// Size: 2, Type: INT, Flags: PUBLIC
    BaseAttackTime = ObjectFields::End as u32 + 0x0038,
    /// Size: 1, Type: INT, Flags: PRIVATE
    RangedAttackTime = ObjectFields::End as u32 + 0x003A,
    /// Size: 1, Type: FLOAT, Flags: PUBLIC
    BoundingRadius = ObjectFields::End as u32 + 0x003B,
    /// Size: 1, Type: FLOAT, Flags: PUBLIC
    CombatReach = ObjectFields::End as u32 + 0x003C,
    /// Size: 1, Type: INT, Flags: PUBLIC
    DisplayId = ObjectFields::End as u32 + 0x003D,
    /// Size: 1, Type: INT, Flags: PUBLIC
    NativeDisplayId = ObjectFields::End as u32 + 0x003E,
    /// Size: 1, Type: INT, Flags: PUBLIC
    MountDisplayId = ObjectFields::End as u32 + 0x003F,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER, PARTY_LEADER
    MinDamage = ObjectFields::End as u32 + 0x0040,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER, PARTY_LEADER
    MaxDamage = ObjectFields::End as u32 + 0x0041,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER, PARTY_LEADER
    MinOffHandDamage = ObjectFields::End as u32 + 0x0042,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER, PARTY_LEADER
    MaxOffHandDamage = ObjectFields::End as u32 + 0x0043,
    /// Size: 1, Type: BYTES, Flags: PUBLIC
    Bytes1 = ObjectFields::End as u32 + 0x0044,
    /// Size: 1, Type: INT, Flags: PUBLIC
    PetNumber = ObjectFields::End as u32 + 0x0045,
    /// Size: 1, Type: INT, Flags: PUBLIC
    PetNameTimestamp = ObjectFields::End as u32 + 0x0046,
    /// Size: 1, Type: INT, Flags: OWNER
    PetExperience = ObjectFields::End as u32 + 0x0047,
    /// Size: 1, Type: INT, Flags: OWNER
    PetNextLevelExp = ObjectFields::End as u32 + 0x0048,
    /// Size: 1, Type: INT, Flags: DYNAMIC
    DynamicFlags = ObjectFields::End as u32 + 0x0049,
    /// Size: 1, Type: FLOAT, Flags: PUBLIC
    ModCastSpeed = ObjectFields::End as u32 + 0x004A,
    /// Size: 1, Type: INT, Flags: PUBLIC
    CreatedBySpell = ObjectFields::End as u32 + 0x004B,
    /// Size: 1, Type: INT, Flags: DYNAMIC
    NpcFlags = ObjectFields::End as u32 + 0x004C,
    /// Size: 1, Type: INT, Flags: PUBLIC
    NpcEmoteState = ObjectFields::End as u32 + 0x004D,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    Stat0 = ObjectFields::End as u32 + 0x004E,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    Stat1 = ObjectFields::End as u32 + 0x004F,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    Stat2 = ObjectFields::End as u32 + 0x0050,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    Stat3 = ObjectFields::End as u32 + 0x0051,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    Stat4 = ObjectFields::End as u32 + 0x0052,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    PosStat0 = ObjectFields::End as u32 + 0x0053,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    PosStat1 = ObjectFields::End as u32 + 0x0054,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    PosStat2 = ObjectFields::End as u32 + 0x0055,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    PosStat3 = ObjectFields::End as u32 + 0x0056,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    PosStat4 = ObjectFields::End as u32 + 0x0057,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    NegStat0 = ObjectFields::End as u32 + 0x0058,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    NegStat1 = ObjectFields::End as u32 + 0x0059,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    NegStat2 = ObjectFields::End as u32 + 0x005A,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    NegStat3 = ObjectFields::End as u32 + 0x005B,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    NegStat4 = ObjectFields::End as u32 + 0x005C,
    /// Size: 7, Type: INT, Flags: PRIVATE, OWNER, PARTY_LEADER
    Resistances = ObjectFields::End as u32 + 0x005D,
    /// Size: 7, Type: INT, Flags: PRIVATE, OWNER
    ResistanceBuffModsPositive = ObjectFields::End as u32 + 0x0064,
    /// Size: 7, Type: INT, Flags: PRIVATE, OWNER
    ResistanceBuffModsNegative = ObjectFields::End as u32 + 0x006B,
    /// Size: 1, Type: INT, Flags: PUBLIC
    BaseMana = ObjectFields::End as u32 + 0x0072,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    BaseHealth = ObjectFields::End as u32 + 0x0073,
    /// Size: 1, Type: BYTES, Flags: PUBLIC
    Bytes2 = ObjectFields::End as u32 + 0x0074,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    AttackPower = ObjectFields::End as u32 + 0x0075,
    /// Size: 1, Type: TWO_SHORT, Flags: PRIVATE, OWNER
    AttackPowerMods = ObjectFields::End as u32 + 0x0076,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER
    AttackPowerMultiplier = ObjectFields::End as u32 + 0x0077,
    /// Size: 1, Type: INT, Flags: PRIVATE, OWNER
    RangedAttackPower = ObjectFields::End as u32 + 0x0078,
    /// Size: 1, Type: TWO_SHORT, Flags: PRIVATE, OWNER
    RangedAttackPowerMods = ObjectFields::End as u32 + 0x0079,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER
    RangedAttackPowerMultiplier = ObjectFields::End as u32 + 0x007A,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER
    MinRangedDamage = ObjectFields::End as u32 + 0x007B,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER
    MaxRangedDamage = ObjectFields::End as u32 + 0x007C,
    /// Size: 7, Type: INT, Flags: PRIVATE, OWNER
    PowerCostModifier = ObjectFields::End as u32 + 0x007D,
    /// Size: 7, Type: FLOAT, Flags: PRIVATE, OWNER
    PowerCostMultiplier = ObjectFields::End as u32 + 0x0084,
    /// Size: 1, Type: FLOAT, Flags: PRIVATE, OWNER
    MaxHealthModifier = ObjectFields::End as u32 + 0x008B,
    /// Size: 1, Type: FLOAT, Flags: PUBLIC
    HoverHeight = ObjectFields::End as u32 + 0x008C,
    /// Size: 1, Type: INT, Flags: NONE
    Padding = ObjectFields::End as u32 + 0x008D,
    End = ObjectFields::End as u32 + 0x008E,

    /// Player fields start here
    PlayerDuelArbiter = End + 0x0000,
    PlayerFlags = End + 0x0002,
    PlayerGuildId = End + 0x0003,
    PlayerGuildRank = End + 0x0004,
    PlayerBytes = End + 0x0005,
    PlayerBytes2 = End + 0x0006,
    PlayerBytes3 = End + 0x0007,
    PlayerDuelTeam = End + 0x0008,
    PlayerGuildTimestamp = End + 0x0009,
    PlayerQuestLog11 = End + 0x000A,
    PlayerQuestLog12 = End + 0x000B,
    PlayerQuestLog13 = End + 0x000C,
    PlayerQuestLog14 = End + 0x000E,
    PlayerQuestLog21 = End + 0x000F,
    PlayerQuestLog22 = End + 0x0010,
    PlayerQuestLog23 = End + 0x0011,
    PlayerQuestLog25 = End + 0x0013,
    PlayerQuestLog31 = End + 0x0014,
    PlayerQuestLog32 = End + 0x0015,
    PlayerQuestLog33 = End + 0x0016,
    PlayerQuestLog35 = End + 0x0018,
    PlayerQuestLog41 = End + 0x0019,
    PlayerQuestLog42 = End + 0x001A,
    PlayerQuestLog43 = End + 0x001B,
    PlayerQuestLog45 = End + 0x001D,
    PlayerQuestLog51 = End + 0x001E,
    PlayerQuestLog52 = End + 0x001F,
    PlayerQuestLog53 = End + 0x0020,
    PlayerQuestLog55 = End + 0x0022,
    PlayerQuestLog61 = End + 0x0023,
    PlayerQuestLog62 = End + 0x0024,
    PlayerQuestLog63 = End + 0x0025,
    PlayerQuestLog65 = End + 0x0027,
    PlayerQuestLog71 = End + 0x0028,
    PlayerQuestLog72 = End + 0x0029,
    PlayerQuestLog73 = End + 0x002A,
    PlayerQuestLog75 = End + 0x002C,
    PlayerQuestLog81 = End + 0x002D,
    PlayerQuestLog82 = End + 0x002E,
    PlayerQuestLog83 = End + 0x002F,
    PlayerQuestLog85 = End + 0x0031,
    PlayerQuestLog91 = End + 0x0032,
    PlayerQuestLog92 = End + 0x0033,
    PlayerQuestLog93 = End + 0x0034,
    PlayerQuestLog95 = End + 0x0036,
    PlayerQuestLog101 = End + 0x0037,
    PlayerQuestLog102 = End + 0x0038,
    PlayerQuestLog103 = End + 0x0039,
    PlayerQuestLog105 = End + 0x003B,
    PlayerQuestLog111 = End + 0x003C,
    PlayerQuestLog112 = End + 0x003D,
    PlayerQuestLog113 = End + 0x003E,
    PlayerQuestLog115 = End + 0x0040,
    PlayerQuestLog121 = End + 0x0041,
    PlayerQuestLog122 = End + 0x0042,
    PlayerQuestLog123 = End + 0x0043,
    PlayerQuestLog125 = End + 0x0045,
    PlayerQuestLog131 = End + 0x0046,
    PlayerQuestLog132 = End + 0x0047,
    PlayerQuestLog133 = End + 0x0048,
    PlayerQuestLog135 = End + 0x004A,
    PlayerQuestLog141 = End + 0x004B,
    PlayerQuestLog142 = End + 0x004C,
    PlayerQuestLog143 = End + 0x004D,
    PlayerQuestLog145 = End + 0x004F,
    PlayerQuestLog151 = End + 0x0050,
    PlayerQuestLog152 = End + 0x0051,
    PlayerQuestLog153 = End + 0x0052,
    PlayerQuestLog155 = End + 0x0054,
    PlayerQuestLog161 = End + 0x0055,
    PlayerQuestLog162 = End + 0x0056,
    PlayerQuestLog163 = End + 0x0057,
    PlayerQuestLog165 = End + 0x0059,
    PlayerQuestLog171 = End + 0x005A,
    PlayerQuestLog172 = End + 0x005B,
    PlayerQuestLog173 = End + 0x005C,
    PlayerQuestLog175 = End + 0x005E,
    PlayerQuestLog181 = End + 0x005F,
    PlayerQuestLog182 = End + 0x0060,
    PlayerQuestLog183 = End + 0x0061,
    PlayerQuestLog185 = End + 0x0063,
    PlayerQuestLog191 = End + 0x0064,
    PlayerQuestLog192 = End + 0x0065,
    PlayerQuestLog193 = End + 0x0066,
    PlayerQuestLog195 = End + 0x0068,
    PlayerQuestLog201 = End + 0x0069,
    PlayerQuestLog202 = End + 0x006A,
    PlayerQuestLog203 = End + 0x006B,
    PlayerQuestLog205 = End + 0x006D,
    PlayerQuestLog211 = End + 0x006E,
    PlayerQuestLog212 = End + 0x006F,
    PlayerQuestLog213 = End + 0x0070,
    PlayerQuestLog215 = End + 0x0072,
    PlayerQuestLog221 = End + 0x0073,
    PlayerQuestLog222 = End + 0x0074,
    PlayerQuestLog223 = End + 0x0075,
    PlayerQuestLog225 = End + 0x0077,
    PlayerQuestLog231 = End + 0x0078,
    PlayerQuestLog232 = End + 0x0079,
    PlayerQuestLog233 = End + 0x007A,
    PlayerQuestLog235 = End + 0x007C,
    PlayerQuestLog241 = End + 0x007D,
    PlayerQuestLog242 = End + 0x007E,
    PlayerQuestLog243 = End + 0x007F,
    PlayerQuestLog245 = End + 0x0081,
    PlayerQuestLog251 = End + 0x0082,
    PlayerQuestLog252 = End + 0x0083,
    PlayerQuestLog253 = End + 0x0084,
    PlayerQuestLog255 = End + 0x0086,
    PlayerVisibleItem1EntryId = End + 0x0087,
    PlayerVisibleItem1Enchantment = End + 0x0088,
    PlayerVisibleItem2EntryId = End + 0x0089,
    PlayerVisibleItem2Enchantment = End + 0x008A,
    PlayerVisibleItem3EntryId = End + 0x008B,
    PlayerVisibleItem3Enchantment = End + 0x008C,
    PlayerVisibleItem4EntryId = End + 0x008D,
    PlayerVisibleItem4Enchantment = End + 0x008E,
    PlayerVisibleItem5EntryId = End + 0x008F,
    PlayerVisibleItem5Enchantment = End + 0x0090,
    PlayerVisibleItem6EntryId = End + 0x0091,
    PlayerVisibleItem6Enchantment = End + 0x0092,
    PlayerVisibleItem7EntryId = End + 0x0093,
    PlayerVisibleItem7Enchantment = End + 0x0094,
    PlayerVisibleItem8EntryId = End + 0x0095,
    PlayerVisibleItem8Enchantment = End + 0x0096,
    PlayerVisibleItem9EntryId = End + 0x0097,
    PlayerVisibleItem9Enchantment = End + 0x0098,
    PlayerVisibleItem10EntryId = End + 0x0099,
    PlayerVisibleItem10Enchantment = End + 0x009A,
    PlayerVisibleItem11EntryId = End + 0x009B,
    PlayerVisibleItem11Enchantment = End + 0x009C,
    PlayerVisibleItem12EntryId = End + 0x009D,
    PlayerVisibleItem12Enchantment = End + 0x009E,
    PlayerVisibleItem13EntryId = End + 0x009F,
    PlayerVisibleItem13Enchantment = End + 0x00A0,
    PlayerVisibleItem14EntryId = End + 0x00A1,
    PlayerVisibleItem14Enchantment = End + 0x00A2,
    PlayerVisibleItem15EntryId = End + 0x00A3,
    PlayerVisibleItem15Enchantment = End + 0x00A4,
    PlayerVisibleItem16EntryId = End + 0x00A5,
    PlayerVisibleItem16Enchantment = End + 0x00A6,
    PlayerVisibleItem17EntryId = End + 0x00A7,
    PlayerVisibleItem17Enchantment = End + 0x00A8,
    PlayerVisibleItem18EntryId = End + 0x00A9,
    PlayerVisibleItem18Enchantment = End + 0x00AA,
    PlayerVisibleItem19EntryId = End + 0x00AB,
    PlayerVisibleItem19Enchantment = End + 0x00AC,
    PlayerChosenTitle = End + 0x00AD,
    PlayerFakeInebriation = End + 0x00AE,
    PlayerFieldPad0 = End + 0x00AF,
    PlayerFieldInvSlotHead = End + 0x00B0,
    PlayerFieldPackSlot1 = End + 0x00DE,
    PlayerFieldBankSlot1 = End + 0x00FE,
    PlayerFieldBankBagSlot1 = End + 0x0136,
    PlayerFieldVendorBuybackSlot1 = End + 0x0144,
    PlayerFieldKeyringSlot1 = End + 0x015C,
    PlayerFieldCurrencyTokenSlot1 = End + 0x019C,
    PlayerFarsight = End + 0x01DC,
    PlayerFieldKnownTitles = End + 0x01DE,
    PlayerFieldKnownTitles1 = End + 0x01E0,
    PlayerFieldKnownTitles2 = End + 0x01E2,
    PlayerFieldKnownCurrencies = End + 0x01E4,
    PlayerXp = End + 0x01E6,
    PlayerNextLevelXp = End + 0x01E7,
    PlayerSkillInfo11 = End + 0x01E8,
    PlayerCharacterPoints1 = End + 0x0368,
    PlayerCharacterPoints2 = End + 0x0369,
    PlayerTrackCreatures = End + 0x036A,
    PlayerTrackResources = End + 0x036B,
    PlayerBlockPercentage = End + 0x036C,
    PlayerDodgePercentage = End + 0x036D,
    PlayerParryPercentage = End + 0x036E,
    PlayerExpertise = End + 0x036F,
    PlayerOffhandExpertise = End + 0x0370,
    PlayerCritPercentage = End + 0x0371,
    PlayerRangedCritPercentage = End + 0x0372,
    PlayerOffhandCritPercentage = End + 0x0373,
    PlayerSpellCritPercentage1 = End + 0x0374,
    PlayerShieldBlock = End + 0x037B,
    PlayerShieldBlockCritPercentage = End + 0x037C,
    PlayerExploredZones1 = End + 0x037D,
    PlayerRestStateExperience = End + 0x03FD,
    PlayerFieldCoinage = End + 0x03FE,
    PlayerFieldModDamageDonePos = End + 0x03FF,
    PlayerFieldModDamageDoneNeg = End + 0x0406,
    PlayerFieldModDamageDonePct = End + 0x040D,
    PlayerFieldModHealingDonePos = End + 0x0414,
    PlayerFieldModHealingPct = End + 0x0415,
    PlayerFieldModHealingDonePct = End + 0x0416,
    PlayerFieldModTargetResistance = End + 0x0417,
    PlayerFieldModTargetPhysicalResistance = End + 0x0418,
    PlayerFieldBytes = End + 0x0419,
    PlayerAmmoId = End + 0x041A,
    PlayerSelfResSpell = End + 0x041B,
    PlayerFieldPvpMedals = End + 0x041C,
    PlayerFieldBuybackPrice1 = End + 0x041D,
    PlayerFieldBuybackTimestamp1 = End + 0x0429,
    PlayerFieldKills = End + 0x0435,
    PlayerFieldTodayContribution = End + 0x0436,
    PlayerFieldYesterdayContribution = End + 0x0437,
    PlayerFieldLifetimeHonorableKills = End + 0x0438,
    PlayerFieldBytes2 = End + 0x0439,
    PlayerFieldWatchedFactionIndex = End + 0x043A,
    PlayerFieldCombatRating1 = End + 0x043B,
    PlayerFieldArenaTeamInfo11 = End + 0x0454,
    PlayerFieldHonorCurrency = End + 0x0469,
    PlayerFieldArenaCurrency = End + 0x046A,
    PlayerFieldMaxLevel = End + 0x046B,
    PlayerFieldDailyQuests1 = End + 0x046C,
    PlayerRuneRegen1 = End + 0x0485,
    PlayerNoReagentCost1 = End + 0x0489,
    PlayerFieldGlyphSlots1 = End + 0x048C,
    PlayerFieldGlyphs1 = End + 0x0492,
    PlayerGlyphsEnabled = End + 0x0498,
    PlayerPetSpellPower = End + 0x0499,
    PlayerEnd = End + 0x049A,
}



// NPC flags enumeration
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum NpcFlags {
    None = 0x00000000,
    Gossip = 0x00000001,
    QuestGiver = 0x00000002,
    Trainer = 0x00000010,
    ClassTrainer = 0x00000020,
    ProfessionTrainer = 0x00000040,
    Vendor = 0x00000080,
    VendorAmmo = 0x00000100,
    VendorFood = 0x00000200,
    VendorPoison = 0x00000400,
    VendorReagent = 0x00000800,
    Repair = 0x00001000,
    FlightMaster = 0x00002000,
    SpiritHealer = 0x00004000,
    SpiritGuide = 0x00008000,
    InnKeeper = 0x00010000,
    Banker = 0x00020000,
    Petitioner = 0x00040000,
    TabardDesigner = 0x00080000,
    BattleMaster = 0x00100000,
    Auctioneer = 0x00200000,
    StableMaster = 0x00400000,
    GuildBanker = 0x00800000,
    Spellclick = 0x01000000,
    PlayerVehicle = 0x02000000,
}


/// Class enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Class {
    Warrior = 1,
    Paladin = 2,
    Hunter = 3,
    Rogue = 4,
    Priest = 5,
    DeathKnight = 6,
    Shaman = 7,
    Mage = 8,
    Warlock = 9,
    Druid = 11,
}

/// Race enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Race {
    Human = 1,
    Orc = 2,
    Dwarf = 3,
    NightElf = 4,
    Undead = 5,
    Tauren = 6,
    Gnome = 7,
    Troll = 8,
    BloodElf = 10,
    Draenei = 11,
}

/// Gender enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Gender {
    Male = 0,
    Female = 1,
}

/// Power type enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PowerType {
    Mana = 0,
    Rage = 1,
    Focus = 2,
    Energy = 3,
    Happiness = 4,
    Rune = 5,
    RunicPower = 6,
}


#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum NamePlateFlag {
    Created = 1,
    Visible = 2,
    CreatedAndVisible = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum CVarFlags {
    None = 0,
    ReadOnly = 0x4,
    CheckTaint = 0x8,
    HideFromUser = 0x40,
    ReadOnlyForUser = 0x100,
}