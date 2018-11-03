const SAMPLE: &'static str = r#"
    #[Foo(I, Don("'t"), Known, What, Should(i = "do"))]
    enum HTTPMethod {
        #[But(it("'s"), seems = "cool")]
        GET(String),
        HEAD{url: String},
        POST,
        PUT,
        #[Do(you(Think), So = "?")]
        DELETE,
        CONNECT,
        OPTIONS,
        TRACE,
    }
"#;

const TARGET: &'static str = r#"Enum {
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
    name: "HTTPMethod",
    variants: [
        Variant {
            name: "GET",
            fields: [
                Field {
                    name: None,
                    type_name: "String",
                    attributes: [],
                    raw_type: Path(
                        TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            String
                                        ),
                                        arguments: None
                                    }
                                ]
                            }
                        }
                    )
                }
            ],
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
            ]
        },
        Variant {
            name: "HEAD",
            fields: [
                Field {
                    name: Some(
                        "url"
                    ),
                    type_name: "String",
                    attributes: [],
                    raw_type: Path(
                        TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            String
                                        ),
                                        arguments: None
                                    }
                                ]
                            }
                        }
                    )
                }
            ],
            attributes: []
        },
        Variant {
            name: "POST",
            fields: [],
            attributes: []
        },
        Variant {
            name: "PUT",
            fields: [],
            attributes: []
        },
        Variant {
            name: "DELETE",
            fields: [],
            attributes: [
                Attribute {
                    name: "Do",
                    sub_nodes: [
                        Attribute {
                            name: "you",
                            sub_nodes: [
                                Attribute {
                                    name: "Think",
                                    sub_nodes: [],
                                    values: []
                                }
                            ],
                            values: []
                        },
                        Attribute {
                            name: "So",
                            sub_nodes: [],
                            values: [
                                Str(
                                    LitStr {
                                        token: Literal {
                                            lit: "?"
                                        }
                                    }
                                )
                            ]
                        }
                    ],
                    values: []
                }
            ]
        },
        Variant {
            name: "CONNECT",
            fields: [],
            attributes: []
        },
        Variant {
            name: "OPTIONS",
            fields: [],
            attributes: []
        },
        Variant {
            name: "TRACE",
            fields: [],
            attributes: []
        }
    ]
}"#;

#[test]
fn basic_test() {
    use proc_macro_helper::*;

    let derive_input = syn::parse_str::<syn::DeriveInput>(SAMPLE).unwrap();
    let target_enum = Enum::parse(&derive_input);
    debug_assert_eq!(&format!("{:#?}", target_enum), TARGET)
}
