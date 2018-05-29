extern crate proc_macro_helper;
extern crate syn;

const SAMPLE: &'static str = r#"
    #[Foo(I, Don("'t"), Known, What, Should(i = "do"))]
    struct User {
        #[But(it("'s"), seems = "cool")]
        pub name: String,
        pub age: Option<u16>,
        pub gender: ::some::thing::about::Gender,
    }
"#;

const TARGET: &'static str = r#"Struct {
    attributes: [
        Attribute {
            name: "Foo",
            sub_nodes: [
                Attribute {
                    name: "I",
                    sub_nodes: [],
                    values: []
                },
                Attribute {
                    name: "Don",
                    sub_nodes: [],
                    values: [
                        Str(
                            LitStr {
                                token: Literal {
                                    lit: "'t"
                                }
                            }
                        )
                    ]
                },
                Attribute {
                    name: "Known",
                    sub_nodes: [],
                    values: []
                },
                Attribute {
                    name: "What",
                    sub_nodes: [],
                    values: []
                },
                Attribute {
                    name: "Should",
                    sub_nodes: [
                        Attribute {
                            name: "i",
                            sub_nodes: [],
                            values: [
                                Str(
                                    LitStr {
                                        token: Literal {
                                            lit: "do"
                                        }
                                    }
                                )
                            ]
                        }
                    ],
                    values: []
                }
            ],
            values: []
        }
    ],
    name: "User",
    fields: [
        Field {
            name: Some(
                "name"
            ),
            type_name: "String",
            attributes: [
                Attribute {
                    name: "But",
                    sub_nodes: [
                        Attribute {
                            name: "it",
                            sub_nodes: [],
                            values: [
                                Str(
                                    LitStr {
                                        token: Literal {
                                            lit: "'s"
                                        }
                                    }
                                )
                            ]
                        },
                        Attribute {
                            name: "seems",
                            sub_nodes: [],
                            values: [
                                Str(
                                    LitStr {
                                        token: Literal {
                                            lit: "cool"
                                        }
                                    }
                                )
                            ]
                        }
                    ],
                    values: []
                }
            ],
            raw_type: Path(
                TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: [
                            PathSegment {
                                ident: Ident {
                                    term: Term {
                                        sym: String
                                    }
                                },
                                arguments: None
                            }
                        ]
                    }
                }
            )
        },
        Field {
            name: Some(
                "age"
            ),
            type_name: "Option<u16>",
            attributes: [],
            raw_type: Path(
                TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: [
                            PathSegment {
                                ident: Ident {
                                    term: Term {
                                        sym: Option
                                    }
                                },
                                arguments: AngleBracketed(
                                    AngleBracketedGenericArguments {
                                        colon2_token: None,
                                        lt_token: Lt,
                                        args: [
                                            Type(
                                                Path(
                                                    TypePath {
                                                        qself: None,
                                                        path: Path {
                                                            leading_colon: None,
                                                            segments: [
                                                                PathSegment {
                                                                    ident: Ident {
                                                                        term: Term {
                                                                            sym: u16
                                                                        }
                                                                    },
                                                                    arguments: None
                                                                }
                                                            ]
                                                        }
                                                    }
                                                )
                                            )
                                        ],
                                        gt_token: Gt
                                    }
                                )
                            }
                        ]
                    }
                }
            )
        },
        Field {
            name: Some(
                "gender"
            ),
            type_name: "::some::thing::about::Gender",
            attributes: [],
            raw_type: Path(
                TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: Some(
                            Colon2
                        ),
                        segments: [
                            (
                                PathSegment {
                                    ident: Ident {
                                        term: Term {
                                            sym: some
                                        }
                                    },
                                    arguments: None
                                },
                                Colon2
                            ),
                            (
                                PathSegment {
                                    ident: Ident {
                                        term: Term {
                                            sym: thing
                                        }
                                    },
                                    arguments: None
                                },
                                Colon2
                            ),
                            (
                                PathSegment {
                                    ident: Ident {
                                        term: Term {
                                            sym: about
                                        }
                                    },
                                    arguments: None
                                },
                                Colon2
                            ),
                            PathSegment {
                                ident: Ident {
                                    term: Term {
                                        sym: Gender
                                    }
                                },
                                arguments: None
                            }
                        ]
                    }
                }
            )
        }
    ]
}"#;

#[test]
fn basic_test() {
    use self::proc_macro_helper::prelude::*;

    let derive_input = syn::parse_str::<syn::DeriveInput>(SAMPLE).unwrap();
    let target_struct = Struct::parse(&derive_input);
    debug_assert_eq!(&format!("{:#?}", target_struct), TARGET)
}
