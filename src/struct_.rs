use attribute::*;
use field::*;
use syn::DeriveInput;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Struct {
    pub attribute: Attribute,
    pub name: String,
    pub fields: Vec<Field>,
}

impl From<DeriveInput> for Struct {
    fn from(source: DeriveInput) -> Self {
        let mut result = Self::default();

        result.name = source.ident.to_string();

        source
            .attrs
            .iter()
            .for_each(|x| result.attribute.push(&x.interpret_meta().unwrap()));

        let fields: ::syn::Fields = match &source.data {
            ::syn::Data::Struct(x) => x.fields.clone(),
            _ => panic!("only support strutc"),
        };
        result.fields = match fields {
            ::syn::Fields::Named(x) => x.named.iter().map(|x| Field::parse(x)).collect(),
            ::syn::Fields::Unnamed(x) => x.unnamed.iter().map(|x| Field::parse(x)).collect(),
            _ => unimplemented!(),
        };

        result
    }
}
