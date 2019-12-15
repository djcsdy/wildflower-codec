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
    GoToLabel(GoToLabel)
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