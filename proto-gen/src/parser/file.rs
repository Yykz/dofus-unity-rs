use pest::Parser;

use super::{common::comment::Comment, CSharpParser, DataStruct, Error, Rule};

#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub members: Vec<DataStruct>,
}

impl ParsedFile {
    pub fn try_from_dump(input: &str) -> Result<Self, Box<pest::error::Error<Rule>>> {
        let mut parsed = CSharpParser::parse(Rule::file, input)?;

        let mut members = vec![];
        let mut last_comment: Option<Comment> = None;

        for member in parsed.next().unwrap().into_inner() {
            match member.as_rule() {
                Rule::COMMENT => {
                    last_comment = Some(Comment::try_from(member).unwrap());
                }
                Rule::fileMember => match DataStruct::try_from(member) {
                    Ok(mut member) => {
                        member.namespace = last_comment
                            .take()
                            .and_then(|c| c.try_extract_namespace().map(ToOwned::to_owned));
                        members.push(member)
                    }
                    Err(e) => panic!("{e}"),
                },
                Rule::EOI => {}
                r => panic!("{:?}", Error::UnexpectedInnerRule(Rule::file, r)),
            }
        }

        Ok(Self { members })
    }
}
