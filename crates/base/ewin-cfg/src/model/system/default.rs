use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgSystem {
    pub os: CfgOS,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgOS {
    pub windows: CfgWindows,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgWindows {
    pub change_output_encoding_utf8: bool,
}
