pub mod do_abc;
pub mod get_url;
pub mod go_to_frame;
pub mod go_to_label;
pub mod push;
pub mod set_target;
pub mod wait_for_frame;

use crate::decode::tags::common::string::String;
use get_url::GetUrl;
use go_to_frame::GoToFrame;
use go_to_label::GoToLabel;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use push::Push;
use set_target::SetTarget;
use wait_for_frame::WaitForFrame;

#[derive(Clone, PartialEq, Debug)]
pub struct DoActionTag {
    pub actions: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DoInitActionTag {
    pub sprite_id: u16,
    pub actions: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
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
    RandomNumber,
    CallFunction,
    CallMethod,
    ConstantPool(ConstantPool),
    DefineFunction(DefineFunction),
    DefineLocal,
    DefineLocal2,
    Delete,
    Delete2,
    Enumerate,
    Equals2,
    GetMember,
    InitArray,
    InitObject,
    NewMethod,
    NewObject,
    SetMember,
    TargetPath,
    With(With),
    ToNumber,
    ToString,
    TypeOf,
    Add2,
    Less2,
    Modulo,
    BitAnd,
    BitLShift,
    BitOr,
    BitRShift,
    BitURShift,
    BitXor,
    Decrement,
    Increment,
    PushDuplicate,
    Return,
    StackSwap,
    StoreRegister(StoreRegister),
    InstanceOf,
    Enumerate2,
    StrictEquals,
    Greater,
    StringGreater,
    DefineFunction2(DefineFunction2),
    Extends,
    CastOp,
    ImplementsOp,
    Try(Try),
    Throw,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PushValue {
    String(String),
    Float(f32),
    Null,
    Undefined,
    RegisterNumber(u8),
    Boolean(bool),
    Double(f64),
    Integer(u32),
    Constant(u16),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Jump {
    pub offset: i16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct If {
    pub offset: i16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl2 {
    pub send_vars_method: SendVarsMethod,
    pub load_target: LoadTarget,
    pub load_variables: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SendVarsMethod {
    None = 0,
    Get = 1,
    Post = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadTarget {
    BrowserWindow = 0,
    PathToSprite = 1,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GoToFrame2 {
    pub play: bool,
    pub scene_bias: u16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WaitForFrame2 {
    pub skip_count: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ConstantPool {
    pub constant_pool: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction {
    pub function_name: String,
    pub params: Vec<String>,
    pub body: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct With {
    pub body: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct StoreRegister {
    pub register_number: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineFunction2 {
    pub function_name: String,
    pub register_count: u8,
    pub preload_parent: bool,
    pub preload_root: bool,
    pub suppress_super: bool,
    pub preload_super: bool,
    pub suppress_arguments: bool,
    pub preload_arguments: bool,
    pub suppress_this: bool,
    pub preload_this: bool,
    pub preload_global: bool,
    pub parameters: Vec<RegisterParam>,
    pub body: Vec<ActionRecord>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RegisterParam {
    Register(u8),
    Name(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Try {
    pub catch_parameter: RegisterParam,
    pub try_body: Vec<ActionRecord>,
    pub catch_body: Option<Vec<ActionRecord>>,
    pub finally_body: Option<Vec<ActionRecord>>,
}
