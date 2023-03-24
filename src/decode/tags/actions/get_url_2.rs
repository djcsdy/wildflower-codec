use crate::decode::tags::actions::load_target::LoadTarget;
use crate::decode::tags::actions::send_vars_method::SendVarsMethod;

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl2 {
    pub send_vars_method: SendVarsMethod,
    pub load_target: LoadTarget,
    pub load_variables: bool,
}
