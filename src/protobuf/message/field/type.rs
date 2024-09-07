use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Type {
    Scalar(Scalar),
    Custom(TypeIdent),
}

impl<T: AsRef<str>> From<T> for Type {
    fn from(value: T) -> Self {
        if let Ok(scalar) = Scalar::try_from(value.as_ref()) {
            Self::Scalar(scalar)
        } else {
            Self::Custom(TypeIdent {
                name: value.as_ref().to_string(),
            })
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Scalar(scalar) => write!(f, "{scalar}"),
            Type::Custom(custom) => write!(f, "{}", custom.name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeIdent {
    pub name: String,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum Scalar {
    double,
    float,
    int32,
    int64,
    uint32,
    uint64,
    sint32,
    sint64,
    fixed32,
    fixed64,
    sfixed32,
    sfixed64,
    bool,
    string,
    bytes,
}

impl TryFrom<&str> for Scalar {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "double" => Ok(Scalar::double),
            "float" => Ok(Scalar::float),
            "int" => Ok(Scalar::int32),
            "long" => Ok(Scalar::int64),
            "uint" => Ok(Scalar::uint32),
            "ulong" => Ok(Scalar::uint64),
            //"sint32" => Ok(Scalar::sint32),
            //"sint64" => Ok(Scalar::sint64),
            //"fixed32" => Ok(Scalar::fixed32),
            //"fixed64" => Ok(Scalar::fixed64),
            //"sfixed32" => Ok(Scalar::sfixed32),
            //"sfixed64" => Ok(Scalar::sfixed64),
            "bool" => Ok(Scalar::bool),
            "string" => Ok(Scalar::string),
            "ByteString" => Ok(Scalar::bytes),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Scalar::double => "double",
            Scalar::float => "float",
            Scalar::int32 => "int32",
            Scalar::int64 => "int64",
            Scalar::uint32 => "uint32",
            Scalar::uint64 => "uint64",
            Scalar::sint32 => "sint32",
            Scalar::sint64 => "sint64",
            Scalar::fixed32 => "fixed32",
            Scalar::fixed64 => "fixed64",
            Scalar::sfixed32 => "sfixed32",
            Scalar::sfixed64 => "sfixed64",
            Scalar::bool => "bool",
            Scalar::string => "string",
            Scalar::bytes => "bytes",
        };
        write!(f, "{}", s)
    }
}
