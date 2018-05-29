extern crate proc_macro;
extern crate proc_macro_helper;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro_helper::prelude::*;
use syn::Data;
use syn::DeriveInput;

#[proc_macro_derive(Table1, attributes(PrimaryKey))]
pub fn table1(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse::<DeriveInput>(input).unwrap();
    let data_struct = if let Data::Struct(ref data) = derive_input.data {
        data
    } else {
        unreachable!();
    };

    let primary_field_idents: Vec<String> = data_struct
        .fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .find(|x| x.interpret_meta().unwrap().name() == "PrimaryKey")
                .is_some()
        })
        .map(|x| x.ident.unwrap().to_string())
        .collect();
    let result_len = primary_field_idents.len();

    quote!(const PRIMARY_FIELDS1: [&'static str; #result_len] = [
        #(#primary_field_idents),*
    ];)
        .into()
}

#[proc_macro_derive(Table2, attributes(PrimaryKey))]
pub fn table2(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse::<DeriveInput>(input).unwrap();
    let target_struct: Struct = Struct::parse(&derive_input);
    let primary_field_idents = target_struct
        .fields
        .iter()
        .filter(|x| x.attributes.iter().find(|x| x.name == "PrimaryKey").is_some())
        .map(|x| x.name.clone().unwrap())
        .collect::<Vec<String>>();
    let result_len = primary_field_idents.len();
    quote!(const PRIMARY_FIELDS2: [&'static str; #result_len] = [
        #(#primary_field_idents),*
    ];)
        .into()
}
