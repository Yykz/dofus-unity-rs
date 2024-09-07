use crate::protobuf::DisplayWithTabs;

mod map;
mod normal;
mod r#type;

pub use map::MapField;
pub use normal::NormalField;
pub use r#type::Type;

// https://protobuf.dev/programming-guides/proto3/#using-oneof
#[derive(Debug, Clone)]
pub enum Field {
    Normal(NormalField),
    OneOf(String, Vec<NormalField>),
    Repeated(NormalField),
    Optional(NormalField),
    Map(MapField),
}

impl Field {
    pub fn custom_types(&mut self) -> Vec<&mut String> {
        match self {
            Field::Normal(f) => option_to_vec(f.custom_type()),
            Field::OneOf(_s, f) => f.iter_mut().filter_map(|f| f.custom_type()).collect(),
            Field::Repeated(f) => option_to_vec(f.custom_type()),
            Field::Optional(f) => option_to_vec(f.custom_type()),
            Field::Map(f) => f.custom_type(),
        }
    }
}

impl DisplayWithTabs for Field {
    fn display_with_tabs(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        tabs_count: usize,
    ) -> std::fmt::Result {
        let tabs = "\t".repeat(tabs_count);
        match self {
            Field::Normal(field) => write!(f, "{tabs}{field}"),
            Field::OneOf(name, fields) => {
                writeln!(f, "{tabs}oneof {name} {{")?;
                for field in fields {
                    writeln!(f, "{tabs}\t{field}")?;
                }
                write!(f, "{tabs}}}")
            }
            Field::Repeated(field) => write!(f, "{tabs}repeated {field}"),
            Field::Optional(field) => write!(f, "{tabs}optional {field}"),
            Field::Map(field) => write!(f, "{tabs}{field}"),
        }
    }
}

fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
    opt.map_or(vec![], |val| vec![val])
}
