use crate::decode::tags::actions::send_vars_method::SendVarsMethod;
use crate::decode::tags::actions::LoadTarget;

#[derive(Clone, PartialEq, Debug)]
pub struct GetUrl2 {
    pub send_vars_method: SendVarsMethod,
    pub load_target: LoadTarget,
    pub load_variables: bool,
}
