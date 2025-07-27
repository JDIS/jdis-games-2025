use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FirewallPattern {
    OneCorner,
    FourCorner,
    Middle,
    None,
}
