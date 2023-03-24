pub mod constant_pool;
pub mod define_function;
pub mod define_function_2;
pub mod do_abc;
pub mod get_url;
pub mod get_url_2;
pub mod go_to_frame;
pub mod go_to_frame_2;
pub mod go_to_label;
pub mod r#if;
pub mod jump;
pub mod load_target;
pub mod push;
pub mod push_value;
pub mod register_param;
pub mod send_vars_method;
pub mod set_target;
pub mod store_register;
pub mod wait_for_frame;
pub mod wait_for_frame_2;
pub mod with;

use constant_pool::ConstantPool;
use define_function::DefineFunction;
use define_function_2::DefineFunction2;
use get_url::GetUrl;
use get_url_2::GetUrl2;
use go_to_frame::GoToFrame;
use go_to_frame_2::GoToFrame2;
use go_to_label::GoToLabel;
use jump::Jump;
use push::Push;
use r#if::If;
use register_param::RegisterParam;
use set_target::SetTarget;
use store_register::StoreRegister;
use wait_for_frame::WaitForFrame;
use wait_for_frame_2::WaitForFrame2;
use with::With;

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
pub struct Try {
    pub catch_parameter: RegisterParam,
    pub try_body: Vec<ActionRecord>,
    pub catch_body: Option<Vec<ActionRecord>>,
    pub finally_body: Option<Vec<ActionRecord>>,
}
