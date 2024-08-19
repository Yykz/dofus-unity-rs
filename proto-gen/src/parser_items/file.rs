use pest::iterators::Pair;

use crate::Rule;

use super::{Class, Namespace};

mod import;
use import::Import;

#[derive(Debug)]
pub struct File {
    pub _imports: Vec<Import>,
    pub content: FileContent,
}

#[derive(Debug)]
pub enum FileContent {
    Class(Class),
    Namespace(Namespace),
}

impl TryFrom<Pair<'_, Rule>> for File {
    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if Rule::file != pair.as_rule() {
            return Err(());
        }
        let mut imports = vec![];
        let mut content = None;
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::EOI | Rule::WHITESPACE => {}
                Rule::COMMENT => {}
                Rule::import => {
                    imports.push(Import::from(inner_pair));
                }
                Rule::class => {
                    content = Some(FileContent::Class(Class::from(inner_pair)));
                }
                Rule::namespace => {
                    content = Some(FileContent::Namespace(Namespace::try_from(inner_pair)?));
                }
                _ => return Err(()),
            }
        }

        let content = match content {
            Some(c) => c,
            None => return Err(()),
        };

        Ok(Self { _imports: imports, content })
    }

    type Error = ();
}
