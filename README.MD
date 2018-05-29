[![build status](https://api.travis-ci.org/DCjanus/proc_macro_helper.svg?branch=master)](https://travis-ci.org/DCjanus/proc_macro_helper)

# Procedural Macros Helper

Since 1.15.0(2017-02-02), basic procedural macros allowing custom `#[derive]`. To use it, you could assign a function as handler, this handler take a `TokenStream` for target `Struct` or `Enum`.

But read directly from `TokenStream` is cumbersome, instead, you can read it via `syn`, a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code. 

But for generality, `syn` is very complex. In your code, there must be a lot noise:

```rust
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
```

With this repo, your code could be:

```rust
use proc_macro_helper::prelude;

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

```

# Current status

+ Need more test cases.
+ Need more feature request.
+ Need more bug reports.

# TODO

+ Implement helper for enum
+ Remove dependency on `quote`