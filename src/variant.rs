use crate::{attribute::Attribute, field::Field};
use syn;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

impl Variant {
    pub fn parse(source: &::syn::Variant) -> Self {
        let mut result = Variant::default();

        result.name = source.ident.to_string();

        result.attributes = source
            .attrs
            .iter()
            .map(|x| x.interpret_meta().unwrap())
            .map(|x| Attribute::parse(&x))
            .collect();

        result.fields = match source.fields {
            syn::Fields::Unit => Vec::new(),
            ::syn::Fields::Unnamed(ref x) => x.unnamed.iter().map(|x| Field::parse(x)).collect(),
            ::syn::Fields::Named(ref x) => x.named.iter().map(|x| Field::parse(x)).collect(),
        };

        result
    }
}
