use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Treatment {
    Control,
    Off,
    On,
}
