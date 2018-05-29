use attribute::Attribute;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Field {
    pub name: Option<String>,
    pub type_name: String,
    pub attributes: Vec<Attribute>,
    pub raw_type: ::syn::Type,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: None,
            type_name: String::new(),
            attributes: Vec::new(),
            raw_type: default_syn_type(),
        }
    }
}

impl Field {
    pub fn parse(source: &::syn::Field) -> Self {
        let mut result = Self::default();

        result.name = source.ident.map(|x| x.to_string());

        result.type_name = description_type(&source.ty).replace(" ", "");

        result.attributes = source
            .attrs
            .iter()
            .map(|x| x.interpret_meta().unwrap())
            .map(|x| Attribute::parse(&x))
            .collect();

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
