pub struct DefineVideoStreamTag {
    pub character_id: u16,
    pub num_frames: u16,
    pub width: u16,
    pub height: u16,
    pub deblocking: Deblocking,
    pub smoothing: bool,
    pub codec: Codec,
}

pub enum Deblocking {
    VideoPacket = 0b000,
    Off = 0b001,
    Level1 = 0b010,
    Level2 = 0b011,
    Level3 = 0b100,
    Level4 = 0b101,
}

pub enum Codec {
    SorensonH263 = 2,
    ScreenVideo = 3,
    Vp6 = 4,
    Vp6WithAlpha = 5,
}
