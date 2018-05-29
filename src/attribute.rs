use syn::{Lit as Literal, Meta, NestedMeta};

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Attribute {
    pub name: String,
    pub sub_nodes: Vec<Attribute>,
    pub values: Vec<Literal>,
}

impl Attribute {
    pub fn parse(source: &Meta) -> Self {
        let mut result = Attribute {
            name: source.name().to_string(),
            ..Default::default()
        };

        match source {
            Meta::Word(_) => {}
            Meta::List(meta_list) => {
                for nested_meta in meta_list.nested.iter() {
                    match nested_meta {
                        NestedMeta::Meta(meta) => {
                            result.sub_nodes.push(Attribute::parse(meta));
                        }
                        NestedMeta::Literal(lit) => {
                            result.values.push(lit.clone());
                        }
                    }
                }
            }
            Meta::NameValue(meta_name_value) => result.values.push(meta_name_value.lit.clone()),
        };

        result
    }
}
