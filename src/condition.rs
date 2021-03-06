use crate::matcher::Matcher;
use crate::Treatment;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub label: Option<String>,
    pub condition_type: Option<ConditionType>,
    pub matcher_group: Option<MatcherGroup>,
    pub partitions: Vec<Partition>,
}

impl Condition {
    pub fn is_empty(&self) -> bool {
        self.label.is_none()
            && self.condition_type.is_none()
            && self.matcher_group.is_none()
            && self.partitions.is_empty()
    }

    // TODO
    pub fn is_match(&self, key: &str) -> bool {
        let matcher_group = match &self.matcher_group {
            Some(matcher_group) => matcher_group,
            None => return false,
        };

        if matcher_group.matchers.is_empty() {
            return false;
        };

        match &matcher_group.combiner {
            Some(combiner) => match combiner {
                Combiner::And => matcher_group.matchers.iter().all(|m| m.is_match(key)),
                _ => return false,
            },
            None => return false,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ConditionType {
    Whitelist,
    Rollout,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatcherGroup {
    combiner: Option<Combiner>,
    matchers: Vec<Matcher>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Combiner {
    And,
    Or,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Partition {
    pub treatment: Treatment,
    pub size: u32,
}

impl Partition {
    pub fn is_hundred_percent(&self) -> bool {
        self.size == 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let c = Condition {
            label: None,
            condition_type: None,
            matcher_group: None,
            partitions: vec![],
        };

        assert!(c.is_empty());
    }

    #[test]
    fn is_not_empty() {
        let c = Condition {
            label: Some("foo".into()),
            condition_type: None,
            matcher_group: None,
            partitions: vec![],
        };

        assert!(!c.is_empty());
    }
}
