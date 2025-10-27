use fastnbt::Value;

pub enum Attachment {
    Ceiling,
    DoubleWall,
    Floor,
    SingleWall,
}

impl Attachment {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "ceiling" => Some(Attachment::Ceiling),
                "double_wall" => Some(Attachment::DoubleWall),
                "floor" => Some(Attachment::Floor),
                "single_wall" => Some(Attachment::SingleWall),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Attachment::Ceiling => "ceiling",
            Attachment::DoubleWall => "double_wall",
            Attachment::Floor => "floor",
            Attachment::SingleWall => "single_wall",
        }.to_string())
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "x" => Some(Axis::X),
                "y" => Some(Axis::Y),
                "z" => Some(Axis::Z),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }.to_string())
    }
}

pub enum CopperGolemPose {
    Running,
    Sitting,
    Standing,
    Star,
}

impl CopperGolemPose {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "running" => Some(CopperGolemPose::Running),
                "sitting" => Some(CopperGolemPose::Sitting),
                "standing" => Some(CopperGolemPose::Standing),
                "star" => Some(CopperGolemPose::Star),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            CopperGolemPose::Running => "running",
            CopperGolemPose::Sitting => "sitting",
            CopperGolemPose::Standing => "standing",
            CopperGolemPose::Star => "star",
        }.to_string())
    }
}

pub enum CreakingHeartState {
    Awake,
    Dormant,
    Uprooted,
}

impl CreakingHeartState {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "awake" => Some(CreakingHeartState::Awake),
                "dormant" => Some(CreakingHeartState::Dormant),
                "uprooted" => Some(CreakingHeartState::Uprooted),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            CreakingHeartState::Awake => "awake",
            CreakingHeartState::Dormant => "dormant",
            CreakingHeartState::Uprooted => "uprooted",
        }.to_string())
    }
}

pub enum WallConnection {
    False,
    Low,
    None,
    Side,
    Tall,
    True,
    Up,
}

impl WallConnection {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(value) = value {
            return match value {
                Value::String(s) => match s.as_str() {
                    "false" => Some(WallConnection::False),
                    "low" => Some(WallConnection::Low),
                    "none" => Some(WallConnection::None),
                    "side" => Some(WallConnection::Side),
                    "tall" => Some(WallConnection::Tall),
                    "true" => Some(WallConnection::True),
                    "up" => Some(WallConnection::Up),
                    _ => None,
                },
                Value::Byte(b) => match *b {
                    0 => Some(WallConnection::False),
                    1 => Some(WallConnection::True),
                    _ => None,
                },
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            WallConnection::False => "false",
            WallConnection::Low => "low",
            WallConnection::None => "none",
            WallConnection::Side => "side",
            WallConnection::Tall => "tall",
            WallConnection::True => "true",
            WallConnection::Up => "up",
        }.to_string())
    }
}

pub enum Face {
    Ceiling,
    Floor,
    Wall,
}

impl Face {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "ceiling" => Some(Face::Ceiling),
                "floor" => Some(Face::Floor),
                "wall" => Some(Face::Wall),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Face::Ceiling => "ceiling",
            Face::Floor => "floor",
            Face::Wall => "wall",
        }.to_string())
    }
}

pub enum Direction {
    Down,
    East,
    North,
    South,
    Up,
    West,
}

impl Direction {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "down" => Some(Direction::Down),
                "east" => Some(Direction::East),
                "north" => Some(Direction::North),
                "south" => Some(Direction::South),
                "up" => Some(Direction::Up),
                "west" => Some(Direction::West),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Direction::Down => "down",
            Direction::East => "east",
            Direction::North => "north",
            Direction::South => "south",
            Direction::Up => "up",
            Direction::West => "west",
        }.to_string())
    }
}

pub enum Half {
    Bottom,
    Lower,
    Top,
    Upper,
}

impl Half {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "bottom" => Some(Half::Bottom),
                "lower" => Some(Half::Lower),
                "top" => Some(Half::Top),
                "upper" => Some(Half::Upper),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Half::Bottom => "bottom",
            Half::Lower => "lower",
            Half::Top => "top",
            Half::Upper => "upper",
        }.to_string())
    }
}

pub enum Hinge {
    Left,
    Right,
}

impl Hinge {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "left" => Some(Hinge::Left),
                "right" => Some(Hinge::Right),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Hinge::Left => "left",
            Hinge::Right => "right",
        }.to_string())
    }
}

pub enum Instrument {
    Banjo,
    Basedrum,
    Bass,
    Bell,
    Bit,
    Chime,
    CowBell,
    Creeper,
    CustomHead,
    Didgeridoo,
    Dragon,
    Flute,
    Guitar,
    Harp,
    Hat,
    IronXylophone,
    Piglin,
    Pling,
    Skeleton,
    Snare,
    Xylophone,
    Zombie,
}

impl Instrument {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "banjo" => Some(Instrument::Banjo),
                "basedrum" => Some(Instrument::Basedrum),
                "bass" => Some(Instrument::Bass),
                "bell" => Some(Instrument::Bell),
                "bit" => Some(Instrument::Bit),
                "chime" => Some(Instrument::Chime),
                "cow_bell" => Some(Instrument::CowBell),
                "creeper" => Some(Instrument::Creeper),
                "custom_head" => Some(Instrument::CustomHead),
                "didgeridoo" => Some(Instrument::Didgeridoo),
                "dragon" => Some(Instrument::Dragon),
                "flute" => Some(Instrument::Flute),
                "guitar" => Some(Instrument::Guitar),
                "harp" => Some(Instrument::Harp),
                "hat" => Some(Instrument::Hat),
                "iron_xylophone" => Some(Instrument::IronXylophone),
                "piglin" => Some(Instrument::Piglin),
                "pling" => Some(Instrument::Pling),
                "skeleton" => Some(Instrument::Skeleton),
                "snare" => Some(Instrument::Snare),
                "xylophone" => Some(Instrument::Xylophone),
                "zombie" => Some(Instrument::Zombie),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Instrument::Banjo => "banjo",
            Instrument::Basedrum => "basedrum",
            Instrument::Bass => "bass",
            Instrument::Bell => "bell",
            Instrument::Bit => "bit",
            Instrument::Chime => "chime",
            Instrument::CowBell => "cow_bell",
            Instrument::Creeper => "creeper",
            Instrument::CustomHead => "custom_head",
            Instrument::Didgeridoo => "didgeridoo",
            Instrument::Dragon => "dragon",
            Instrument::Flute => "flute",
            Instrument::Guitar => "guitar",
            Instrument::Harp => "harp",
            Instrument::Hat => "hat",
            Instrument::IronXylophone => "iron_xylophone",
            Instrument::Piglin => "piglin",
            Instrument::Pling => "pling",
            Instrument::Skeleton => "skeleton",
            Instrument::Snare => "snare",
            Instrument::Xylophone => "xylophone",
            Instrument::Zombie => "zombie",
        }.to_string())
    }
}

pub enum Leaves {
    Large,
    None,
    Small,
}

impl Leaves {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "large" => Some(Leaves::Large),
                "none" => Some(Leaves::None),
                "small" => Some(Leaves::Small),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Leaves::Large => "large",
            Leaves::None => "none",
            Leaves::Small => "small",
        }.to_string())
    }
}

pub enum Mode {
    Compare,
    Corner,
    Data,
    Load,
    Save,
    Subtract,
}

impl Mode {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "compare" => Some(Mode::Compare),
                "corner" => Some(Mode::Corner),
                "data" => Some(Mode::Data),
                "load" => Some(Mode::Load),
                "save" => Some(Mode::Save),
                "subtract" => Some(Mode::Subtract),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Mode::Compare => "compare",
            Mode::Corner => "corner",
            Mode::Data => "data",
            Mode::Load => "load",
            Mode::Save => "save",
            Mode::Subtract => "subtract",
        }.to_string())
    }
}

pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    EastUp,
    NorthUp,
    SouthUp,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
}

impl Orientation {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "down_east" => Some(Orientation::DownEast),
                "down_north" => Some(Orientation::DownNorth),
                "down_south" => Some(Orientation::DownSouth),
                "down_west" => Some(Orientation::DownWest),
                "east_up" => Some(Orientation::EastUp),
                "north_up" => Some(Orientation::NorthUp),
                "south_up" => Some(Orientation::SouthUp),
                "up_east" => Some(Orientation::UpEast),
                "up_north" => Some(Orientation::UpNorth),
                "up_south" => Some(Orientation::UpSouth),
                "up_west" => Some(Orientation::UpWest),
                "west_up" => Some(Orientation::WestUp),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Orientation::DownEast => "down_east",
            Orientation::DownNorth => "down_north",
            Orientation::DownSouth => "down_south",
            Orientation::DownWest => "down_west",
            Orientation::EastUp => "east_up",
            Orientation::NorthUp => "north_up",
            Orientation::SouthUp => "south_up",
            Orientation::UpEast => "up_east",
            Orientation::UpNorth => "up_north",
            Orientation::UpSouth => "up_south",
            Orientation::UpWest => "up_west",
            Orientation::WestUp => "west_up",
        }.to_string())
    }
}

pub enum Part {
    Foot,
    Head,
}

impl Part {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "foot" => Some(Part::Foot),
                "head" => Some(Part::Head),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Part::Foot => "foot",
            Part::Head => "head",
        }.to_string())
    }
}

pub enum SculkSensorPhase {
    Active,
    Cooldown,
    Inactive,
}

impl SculkSensorPhase {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "active" => Some(SculkSensorPhase::Active),
                "cooldown" => Some(SculkSensorPhase::Cooldown),
                "inactive" => Some(SculkSensorPhase::Inactive),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            SculkSensorPhase::Active => "active",
            SculkSensorPhase::Cooldown => "cooldown",
            SculkSensorPhase::Inactive => "inactive",
        }.to_string())
    }
}

pub enum Shape {
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest,
    EastWest,
    InnerLeft,
    InnerRight,
    NorthEast,
    NorthSouth,
    NorthWest,
    OuterLeft,
    OuterRight,
    SouthEast,
    SouthWest,
    Straight,
}

impl Shape {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "ascending_east" => Some(Shape::AscendingEast),
                "ascending_north" => Some(Shape::AscendingNorth),
                "ascending_south" => Some(Shape::AscendingSouth),
                "ascending_west" => Some(Shape::AscendingWest),
                "east_west" => Some(Shape::EastWest),
                "inner_left" => Some(Shape::InnerLeft),
                "inner_right" => Some(Shape::InnerRight),
                "north_east" => Some(Shape::NorthEast),
                "north_south" => Some(Shape::NorthSouth),
                "north_west" => Some(Shape::NorthWest),
                "outer_left" => Some(Shape::OuterLeft),
                "outer_right" => Some(Shape::OuterRight),
                "south_east" => Some(Shape::SouthEast),
                "south_west" => Some(Shape::SouthWest),
                "straight" => Some(Shape::Straight),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Shape::AscendingEast => "ascending_east",
            Shape::AscendingNorth => "ascending_north",
            Shape::AscendingSouth => "ascending_south",
            Shape::AscendingWest => "ascending_west",
            Shape::EastWest => "east_west",
            Shape::InnerLeft => "inner_left",
            Shape::InnerRight => "inner_right",
            Shape::NorthEast => "north_east",
            Shape::NorthSouth => "north_south",
            Shape::NorthWest => "north_west",
            Shape::OuterLeft => "outer_left",
            Shape::OuterRight => "outer_right",
            Shape::SouthEast => "south_east",
            Shape::SouthWest => "south_west",
            Shape::Straight => "straight",
        }.to_string())
    }
}

pub enum Thickness {
    Base,
    Frustum,
    Middle,
    Tip,
    TipMerge,
}

impl Thickness {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "base" => Some(Thickness::Base),
                "frustum" => Some(Thickness::Frustum),
                "middle" => Some(Thickness::Middle),
                "tip" => Some(Thickness::Tip),
                "tip_merge" => Some(Thickness::TipMerge),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Thickness::Base => "base",
            Thickness::Frustum => "frustum",
            Thickness::Middle => "middle",
            Thickness::Tip => "tip",
            Thickness::TipMerge => "tip_merge",
        }.to_string())
    }
}

pub enum Tilt {
    Full,
    None,
    Partial,
    Unstable,
}

impl Tilt {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "full" => Some(Tilt::Full),
                "none" => Some(Tilt::None),
                "partial" => Some(Tilt::Partial),
                "unstable" => Some(Tilt::Unstable),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            Tilt::Full => "full",
            Tilt::None => "none",
            Tilt::Partial => "partial",
            Tilt::Unstable => "unstable",
        }.to_string())
    }
}

pub enum TrialSpawnerState {
    Active,
    Cooldown,
    EjectingReward,
    Inactive,
    WaitingForPlayers,
    WaitingForRewardEjection,
}

impl TrialSpawnerState {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "active" => Some(TrialSpawnerState::Active),
                "cooldown" => Some(TrialSpawnerState::Cooldown),
                "ejecting_reward" => Some(TrialSpawnerState::EjectingReward),
                "inactive" => Some(TrialSpawnerState::Inactive),
                "waiting_for_players" => Some(TrialSpawnerState::WaitingForPlayers),
                "waiting_for_reward_ejection" => Some(TrialSpawnerState::WaitingForRewardEjection),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            TrialSpawnerState::Active => "active",
            TrialSpawnerState::Cooldown => "cooldown",
            TrialSpawnerState::EjectingReward => "ejecting_reward",
            TrialSpawnerState::Inactive => "inactive",
            TrialSpawnerState::WaitingForPlayers => "waiting_for_players",
            TrialSpawnerState::WaitingForRewardEjection => "waiting_for_reward_ejection",
        }.to_string())
    }
}

pub enum BlockType {
    Bottom,
    Double,
    Left,
    Normal,
    Right,
    Single,
    Sticky,
    Top,
}

impl BlockType {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "bottom" => Some(BlockType::Bottom),
                "double" => Some(BlockType::Double),
                "left" => Some(BlockType::Left),
                "normal" => Some(BlockType::Normal),
                "right" => Some(BlockType::Right),
                "single" => Some(BlockType::Single),
                "sticky" => Some(BlockType::Sticky),
                "top" => Some(BlockType::Top),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            BlockType::Bottom => "bottom",
            BlockType::Double => "double",
            BlockType::Left => "left",
            BlockType::Normal => "normal",
            BlockType::Right => "right",
            BlockType::Single => "single",
            BlockType::Sticky => "sticky",
            BlockType::Top => "top",
        }.to_string())
    }
}

pub enum VaultState {
    Active,
    Ejecting,
    Inactive,
    Unlocking,
}

impl VaultState {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "active" => Some(VaultState::Active),
                "ejecting" => Some(VaultState::Ejecting),
                "inactive" => Some(VaultState::Inactive),
                "unlocking" => Some(VaultState::Unlocking),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            VaultState::Active => "active",
            VaultState::Ejecting => "ejecting",
            VaultState::Inactive => "inactive",
            VaultState::Unlocking => "unlocking",
        }.to_string())
    }
}

pub enum VerticalDirection {
    Down,
    Up,
}

impl VerticalDirection {
    pub fn from_nbt(value: Option<&Value>) -> Option<Self> {
        if let Some(Value::String(s)) = value {
            return match s.as_str() {
                "down" => Some(VerticalDirection::Down),
                "up" => Some(VerticalDirection::Up),
                _ => None,
            };
        }
        None
    }

    pub fn to_nbt(&self) -> Value {
        Value::String(match self {
            VerticalDirection::Down => "down",
            VerticalDirection::Up => "up",
        }.to_string())
    }
}