use std::fmt::Display;

use super::Type;

#[derive(Debug, Clone)]
pub struct NormalField {
    ty: Type,
    name: String,
    number: u32,
}

impl NormalField {
    pub fn custom_type(&mut self) -> Option<&mut String> {
        if let Type::Custom(t) = &mut self.ty {
            Some(&mut t.name)
        } else {
            None
        }
    }

    pub fn new<T: Into<Type>>(ty: T, name: String, number: u32) -> Self {
        Self {
            ty: ty.into(),
            name,
            number,
        }
    }
}

impl Display for NormalField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.ty, self.name, self.number)
    }
}
