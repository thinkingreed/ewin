use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserSystem {
    pub os: CfgUserOS,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserOS {
    pub windows: CfgUserWindows,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserWindows {
    pub change_output_encoding_utf8: Option<bool>,
}
