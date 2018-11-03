use crate::{attribute::*, variant::Variant};
use syn::DeriveInput;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Enum {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub variants: Vec<Variant>,
}

impl Enum {
    pub fn parse(source: &DeriveInput) -> Self {
        let mut result = Enum::default();

        result.name = source.ident.to_string();

        result.attributes = source
            .attrs
            .iter()
            .map(|x| x.interpret_meta().unwrap())
            .map(|x| Attribute::parse(&x))
            .collect();

        let data_enum = match &source.data {
            ::syn::Data::Enum(x) => x,
            _ => panic!("only support enum"),
        };
        let variants_iter: ::syn::punctuated::Iter<::syn::Variant> = data_enum.variants.iter();

        result.variants = variants_iter.map(|x| Variant::parse(x)).collect();

        result
    }
}
