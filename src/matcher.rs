pub trait Matcher {
    fn is_match(&self) -> bool;
}

pub struct AllKeysMatcher;
pub struct WhitelistMatcher {
    whitelist: Vec<String>,
};

impl WhitelistMatcher {
    pub fn new(whitelist: Vec<String>) -> Self {
        Self { whitelist }
    }
}

impl Matcher for AllKeysMatcher {
    fn is_match(&self) -> bool {
        true
    }
}

impl Matcher for WhitelistMatcher {
    fn is_match(&self, value: &str) -> bool {
        self.data.iter().any(|&e| e == value)
    }
}

pub struct NegationMatcher;
