use std::collections::{HashMap, HashSet};
use syn::{Lit as Literal, Meta, NestedMeta};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Attribute {
    pub sub_attributes: HashMap<String, Attribute>,
    pub values: HashSet<Literal>,
}

impl Attribute {
    pub fn ty(&self) -> AttributeType {
        if self.values.is_empty() {
            AttributeType::Pair
        } else if self.sub_attributes.is_empty() {
            AttributeType::None
        } else {
            AttributeType::Nest
        }
    }
    pub fn contains(&self, key: &str) -> bool {
        self.sub_attributes.contains_key(key)
    }

    pub fn push(&mut self, meta: &Meta) {
        let name = meta.name().to_string();
        match meta {
            Meta::Word(_) => {
                self.get_or_create_sub_attribute(name);
            }
            Meta::NameValue(name_value) => {
                self.get_or_create_sub_attribute(name)
                    .values
                    .insert(name_value.lit.clone());
            }
            Meta::List(meta_list) => {
                let mut target = self.get_or_create_sub_attribute(name);

                meta_list.nested.iter().for_each(|x| match x {
                    NestedMeta::Meta(meta) => {
                        target
                            .get_or_create_sub_attribute(meta.name().to_string())
                            .push(meta);
                    }
                    NestedMeta::Literal(lit) => {
                        target.values.insert(lit.clone());
                    }
                });
            }
        }
    }

    pub fn get_or_create_sub_attribute(&mut self, name: impl AsRef<str>) -> &mut Self {
        if !self.sub_attributes.contains_key(name.as_ref()) {
            self.sub_attributes
                .insert(name.as_ref().to_owned(), Attribute::default());
        }
        self.sub_attributes.get_mut(name.as_ref()).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AttributeType {
    None,
    Nest,
    Pair,
}
