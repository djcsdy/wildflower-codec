/// Overrides the default limits for AVM scripts.
#[derive(Clone, PartialEq, Debug)]
pub struct ScriptLimitsTag {
    pub max_recursion_depth: u16,
    pub script_timeout_seconds: u16,
}
