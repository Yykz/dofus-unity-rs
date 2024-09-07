use std::fmt::Display;

use super::Type;

#[derive(Debug, Clone)]
pub struct MapField {
    key_ty: Type,
    val_ty: Type,
    name: String,
    number: u32,
}

impl MapField {
    pub fn new<T: Into<Type>, T2: Into<Type>>(
        key_ty: T,
        val_ty: T2,
        name: String,
        number: u32,
    ) -> Self {
        Self {
            key_ty: key_ty.into(),
            val_ty: val_ty.into(),
            name,
            number,
        }
    }

    pub fn custom_type(&mut self) -> Vec<&mut String> {
        let mut tys = vec![];
        if let Type::Custom(t) = &mut self.key_ty {
            tys.push(&mut t.name)
        }
        if let Type::Custom(t) = &mut self.val_ty {
            tys.push(&mut t.name)
        }

        tys
    }
}

impl Display for MapField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "map<{}, {}> {} = {};",
            self.key_ty, self.val_ty, self.name, self.number
        )
    }
}
