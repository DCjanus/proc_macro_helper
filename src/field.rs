use attribute::Attribute;

#[derive(Debug, Eq, PartialEq)]
pub struct Field {
    pub name: Option<String>,
    pub ty: String,
    pub attribute: Attribute,
    pub raw_type: ::syn::Type,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: None,
            ty: String::new(),
            attribute: Attribute::default(),
            raw_type: default_syn_type(),
        }
    }
}

impl Field {
    pub fn parse(source: &::syn::Field) -> Self {
        let mut result = Self::default();

        result.name = source.ident.map(|x| x.to_string());

        result.ty = description_type(&source.ty).replace(" ", "");

        source
            .attrs
            .iter()
            .for_each(|x| result.attribute.push(&x.interpret_meta().unwrap()));

        result.raw_type = source.ty.clone();

        result
    }
}

fn description_type(source: &::syn::Type) -> String {
    quote!(#source).to_string()
}

fn default_syn_type() -> ::syn::Type {
    ::syn::parse_str("[DCjanu; 12]").unwrap()
}
