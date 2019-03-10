use crate::Treatment;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    label: Option<String>,
    condition_type: Option<ConditionType>,
    matcher_group: MatcherGroup,
    partitions: Vec<Partition>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionType {
    Whitelist,
    Rollout,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatcherGroup {
    combiner: Combiner,
    matchers: Vec<Matcher>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Combiner {
    And,
    Or,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Matcher {
    matcher_type: MatcherType,
    negate: bool,
    whitelist_matcher_data: Option<WhitelistMatcherData>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatcherType {
    Whitelist,
    AllKeys,
    InSegment,
    EqualTo,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    Between,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WhitelistMatcherData {
    whitelist: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Partition {
    treatment: String,
    size: u8,
}
