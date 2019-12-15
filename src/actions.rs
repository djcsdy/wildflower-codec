pub struct DoActionTag {
    pub actions: Vec<ActionRecord>
}

pub enum ActionRecord {
    GoToFrame(GoToFrame),
    GetUrl(GetUrl),
    NextFrame,
    PreviousFrame,
    Play,
    Stop,
    ToggleQuality,
    StopSounds,
    WaitForFrame(WaitForFrame),
    SetTarget(SetTarget),
    GoToLabel(GoToLabel),
    Push(Push),
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    Less,
    And,
    Or,
    Not,
    StringEquals,
    StringLength,
    StringAdd,
    StringExtract,
    StringLess,
    MbStringLength,
    MbStringExtract,
    ToInteger,
    CharToAscii,
    AsciiToChar,
    MbCharToAscii,
    MbAsciiToChar,
    Jump(Jump),
    If(If),
    Call,
    GetVariable,
    SetVariable,
    GetUrl2(GetUrl2),
    GoToFrame2(GoToFrame2),
    SetTarget2,
    GetProperty,
    SetProperty,
    CloneSprite,
    RemoveSprite,
    StartDrag,
    EndDrag,
    WaitForFrame2(WaitForFrame2),
    Trace,
    GetTime,
    RandomNumber
}

pub struct GoToFrame {
    pub frame: u16
}

pub struct GetUrl {
    pub url: String,
    pub target: String
}

pub struct WaitForFrame {
    pub frame: u16,
    pub skip_count: u8
}

pub struct SetTarget {
    pub target_name: String
}

pub struct GoToLabel {
    pub label: String
}

pub struct Push {
    pub value: PushValue
}

pub enum PushValue {
    String(String),
    Float(f32),
    RegisterNumber(u8),
    Boolean(bool),
    Double(f64),
    Integer(u32),
    Constant(u16)
}

pub struct Jump {
    pub offset: i16
}

pub struct If {
    pub offset: i16
}

pub struct GetUrl2 {
    pub send_vars_method: SendVarsMethod,
    pub load_target: LoadTarget,
    pub load_variables: bool
}

pub enum SendVarsMethod {
    None = 0,
    Get = 1,
    Post = 2
}

pub enum LoadTarget {
    BrowserWindow = 0,
    PathToSprite = 1
}

pub struct GoToFrame2 {
    pub play: bool,
    pub scene_bias: u16
}

pub struct WaitForFrame2 {
    pub skip_count: u8
}