# Procedural Macros Helper

Since 1.15.0(2017-02-02), basic procedural macros allowing custom `#[derive]`. To use it, you could assign a function as handler, this handler take a `TokenStream` for target `Struct` or `Enum`.

But read directly from `TokenStream` is cumbersome, instead, you can read it via `syn`, a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code. 

But for generality, `syn` is very complex. In your code, there must be a lot noise:

```rust
#[proc_macro_derive(Table, attributes(UniqueKey))]
pub fn table1(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse::<DeriveInput>(input).unwrap();
    let data_struct = if let Data::Struct(ref data) = derive_input.data {
        data
    } else {
        unreachable!();
    };

    let primary_field_idents: Vec<Ident> = data_struct
        .fields
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .map(|x| x.interpret_meta().unwrap())
                .map(|x| x.name())
                .collect::<HashSet<Ident>>()
                .contains(&"UniqueKey".into())
        })
        .map(|x| x.ident.unwrap())
        .collect();

    quote!(let primary_fields = vec![
        #(#primary_field_idents),*
    ]).into()
}
```

With this repo, your code could be:

```rust
#[proc_macro_derive(Table, attributes(UniqueKey))]
pub fn table2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = syn::parse::<DeriveInput>(input).unwrap();
    let target_struct = Struct::from(derive_input);

    let primary_field_idents: Vec<Ident> = target_struct
        .fields
        .iter()
        .filter(|x| x.attribute.contains("UniqueKey"))
        .map(|x| syn::parse_str(&x.name.clone().unwrap()).unwrap())
        .collect();

    quote!(let primary_fields = vec![
        #(#primary_field_idents),*
    ]).into()
}
```

# Current status

+ Need more test cases.
+ Need more feature request.
+ Need more bug reports.

# TODO

+ Implement helper for enum
+ Remove dependency on `quote`