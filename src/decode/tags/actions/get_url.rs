use crate::decode::tags::common::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl {
    pub url: String,
    pub target: String,
}
