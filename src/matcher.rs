use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Matcher {
    matcher_type: MatcherType,
    negate: bool,
    whitelist_matcher_data: Option<WhitelistMatcherData>,
}

impl Matcher {
    pub fn is_match(&self, key: &str) -> bool {
        match self.matcher_type {
            MatcherType::AllKeys => true,
            MatcherType::Whitelist => match &self.whitelist_matcher_data {
                Some(ref data) => data.whitelist.iter().any(|e| e == key),
                None => false,
            },
            _ => {
                unimplemented!("need to implement more matcher types");
            }
        }
    }
}

// pub enum Matcher {
//     AllKeys,
//     Whitelist(Vec<String>),
// }
//
// impl Matcher {
//     pub fn is_match(&self, key: &str) -> bool {
//         match self {
//             Matcher::AllKeys => true,
//             Matcher::Whitelist(data) => data.iter().any(|&e| e == key),
//         }
//     }
// }

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MatcherType {
    AllKeys,
    InSegment,
    Whitelist,
    EqualTo,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Between,
    EqualToSet,
    ContainsAnyOfSet,
    ContainsAllOfSet,
    PartOfSet,
    StartsWith,
    EndsWith,
    ContainsString,
    InSplitTreatment,
    EqualToBoolean,
    MatchesString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WhitelistMatcherData {
    whitelist: Vec<String>,
}
