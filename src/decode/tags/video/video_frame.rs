#[derive(Clone, PartialEq, Debug)]
pub struct VideoFrameTag {
    pub stream_id: u16,
    pub frame_num: u16,
    pub video_data: Vec<u8>,
}
