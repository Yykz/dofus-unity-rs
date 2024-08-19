use pest::iterators::Pair;

use crate::{parser_items::Member, Rule};

#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub member: Member,
}

impl TryFrom<Pair<'_, Rule>> for Namespace {
    type Error = ();

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if value.as_rule() != Rule::namespace {
            return Err(());
        }

        let mut name = None;
        let mut member = None;
        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::itemPath => name = Some(pair.as_str().to_string()),
                Rule::namespaceBody => {
                    for pair in pair.into_inner() {
                        member = Some(Member::from(pair));
                    }
                }
                _ => return Err(()),
            }
        }

        match (name, member) {
            (Some(name), Some(member)) => Ok(Self { name, member }),
            _ => Err(()),
        }
    }
}
