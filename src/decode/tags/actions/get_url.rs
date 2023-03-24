use crate::decode::tags::common::string;

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl {
    pub url: string::String,
    pub target: string::String,
}
