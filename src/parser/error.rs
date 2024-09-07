use super::Rule;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid inner rule in {0:?} got {1:?}")]
    UnexpectedInnerRule(Rule, Rule),
    #[error("invalid rule, expected {0:?} got {1:?}")]
    UnexpectedRule(Rule, Rule),
    #[error("invalid token in {0:?} got {1}")]
    InvalidToken(Rule, String),
    #[error("failed to build {0:?}")]
    Builder(Box<dyn std::error::Error>),
}

impl Error {
    pub fn from_builder_error<E: std::error::Error + 'static>(e: E) -> Self {
        Self::Builder(Box::new(e))
    }

    pub fn rule_matches(expected: Rule, got: Rule) -> Result<(), Self> {
        if expected != got {
            Err(Self::UnexpectedRule(expected, got))
        } else {
            Ok(())
        }
    }
}
