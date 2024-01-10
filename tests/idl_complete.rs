const EXAMPLE_SCHEMA: &str = include_str!("./idl_complete.ww");

#[test]
fn test_schema_loader() {
    use webwire_cli::common::FilePosition;
    use webwire_cli::idl::*;
    let result = parse_document(EXAMPLE_SCHEMA);
    assert!(result.is_ok(), "{:?}", result);
    let result = result.unwrap();
    assert_eq!(
        result,
        Document {
            includes: vec![],
            ns: Namespace {
                name: "".to_string(),
                position: FilePosition { line: 1, column: 1 },
                parts: vec![NamespacePart::Namespace(Namespace {
                    name: "user".to_string(),
                    position: FilePosition { line: 1, column: 1 },
                    parts: vec![
                        NamespacePart::Enum(Enum {
                            name: "UserState".to_string(),
                            generics: vec![],
                            position: FilePosition { line: 3, column: 5 },
                            extends: None,
                            variants: vec![
                                EnumVariant {
                                    name: "Active".to_string(),
                                    value_type: None,
                                },
                                EnumVariant {
                                    name: "Inactive".to_string(),
                                    value_type: None,
                                },
                                EnumVariant {
                                    name: "Banned".to_string(),
                                    value_type: None,
                                },
                            ]
                        }),
                        NamespacePart::Struct(Struct {
                            name: "UserRequest".to_string(),
                            position: FilePosition { line: 9, column: 5 },
                            generics: vec![],
                            fields: vec![Field {
                                name: "email".to_string(),
                                position: FilePosition {
                                    line: 10,
                                    column: 9
                                },
                                type_: Type::Ref(TypeRef {
                                    abs: false,
                                    ns: vec![],
                                    name: "String".to_string(),
                                    generics: vec![]
                                }),
                                optional: false,
                                options: vec![]
                            }]
                        }),
                        NamespacePart::Struct(Struct {
                            name: "Name".to_string(),
                            position: FilePosition {
                                line: 13,
                                column: 5
                            },
                            generics: vec![],
                            fields: vec![
                                Field {
                                    name: "prefix".to_string(),
                                    position: FilePosition {
                                        line: 14,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 14,
                                            column: 25
                                        },
                                        name: "length".to_string(),
                                        value: Value::Range(Some(0), Some(50))
                                    }]
                                },
                                Field {
                                    name: "first_name".to_string(),
                                    position: FilePosition {
                                        line: 15,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 15,
                                            column: 29
                                        },
                                        name: "length".to_string(),
                                        value: Value::Range(Some(0), Some(100))
                                    }]
                                },
                                Field {
                                    name: "middle_name".to_string(),
                                    position: FilePosition {
                                        line: 16,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 16,
                                            column: 30
                                        },
                                        name: "length".to_string(),
                                        value: Value::Range(Some(0), Some(100))
                                    }]
                                },
                                Field {
                                    name: "last_name".to_string(),
                                    position: FilePosition {
                                        line: 17,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![
                                        FieldOption {
                                            position: FilePosition {
                                                line: 17,
                                                column: 28
                                            },
                                            name: "length".to_string(),
                                            value: Value::Range(Some(0), Some(100))
                                        },
                                        FieldOption {
                                            position: FilePosition {
                                                line: 17,
                                                column: 43
                                            },
                                            name: "help".to_string(),
                                            value: Value::String("aka. family name".to_string())
                                        }
                                    ]
                                },
                                Field {
                                    name: "suffix".to_string(),
                                    position: FilePosition {
                                        line: 18,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 18,
                                            column: 25
                                        },
                                        name: "length".to_string(),
                                        value: Value::Range(Some(0), Some(50))
                                    }]
                                },
                                Field {
                                    name: "full_name".to_string(),
                                    position: FilePosition {
                                        line: 19,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![],
                                    }),
                                    optional: false,
                                    options: vec![]
                                }
                            ]
                        }),
                        NamespacePart::Fieldset(Fieldset {
                            name: "NameUpdate".to_string(),
                            generics: vec![],
                            position: FilePosition {
                                line: 22,
                                column: 5
                            },
                            r#struct: TypeRef {
                                abs: false,
                                ns: vec![],
                                name: "Name".to_string(),
                                generics: vec![],
                            },
                            fields: vec![
                                FieldsetField {
                                    name: "prefix".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "first_name".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "middle_name".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "last_name".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "suffix".to_string(),
                                    optional: true
                                }
                            ]
                        }),
                        NamespacePart::Struct(Struct {
                            name: "User".to_string(),
                            position: FilePosition {
                                line: 30,
                                column: 5
                            },
                            generics: vec![],
                            fields: vec![
                                Field {
                                    name: "id".to_string(),
                                    position: FilePosition {
                                        line: 31,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "UUID".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![]
                                },
                                Field {
                                    name: "email".to_string(),
                                    position: FilePosition {
                                        line: 32,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![]
                                },
                                Field {
                                    name: "name".to_string(),
                                    position: FilePosition {
                                        line: 33,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Name".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![]
                                },
                                Field {
                                    name: "password".to_string(),
                                    position: FilePosition {
                                        line: 34,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 34,
                                            column: 27
                                        },
                                        name: "length".to_string(),
                                        value: Value::Range(Some(5), Some(64))
                                    }]
                                },
                                Field {
                                    name: "is_admin".to_string(),
                                    position: FilePosition {
                                        line: 35,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Boolean".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![]
                                }
                            ]
                        }),
                        NamespacePart::Fieldset(Fieldset {
                            name: "UserRead".to_string(),
                            generics: vec![],
                            position: FilePosition {
                                line: 38,
                                column: 5
                            },
                            r#struct: TypeRef {
                                abs: false,
                                ns: vec![],
                                name: "User".to_string(),
                                generics: vec![],
                            },
                            fields: vec![
                                FieldsetField {
                                    name: "id".to_string(),
                                    optional: false
                                },
                                FieldsetField {
                                    name: "email".to_string(),
                                    optional: false
                                },
                                FieldsetField {
                                    name: "is_admin".to_string(),
                                    optional: false
                                },
                                FieldsetField {
                                    name: "name".to_string(),
                                    optional: false
                                }
                            ]
                        }),
                        NamespacePart::Fieldset(Fieldset {
                            name: "UserWrite".to_string(),
                            generics: vec![],
                            position: FilePosition {
                                line: 45,
                                column: 5
                            },
                            r#struct: TypeRef {
                                abs: false,
                                ns: vec![],
                                name: "User".to_string(),
                                generics: vec![],
                            },
                            fields: vec![
                                FieldsetField {
                                    name: "id".to_string(),
                                    optional: false
                                },
                                FieldsetField {
                                    name: "email".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "is_admin".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "name".to_string(),
                                    optional: true
                                },
                                FieldsetField {
                                    name: "password".to_string(),
                                    optional: true
                                }
                            ]
                        }),
                        NamespacePart::Struct(Struct {
                            name: "UserListRequest".to_string(),
                            position: FilePosition {
                                line: 53,
                                column: 5
                            },
                            generics: vec![],
                            fields: vec![
                                Field {
                                    name: "offset".to_string(),
                                    position: FilePosition {
                                        line: 54,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Integer".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: true,
                                    options: vec![
                                        FieldOption {
                                            position: FilePosition {
                                                line: 54,
                                                column: 27
                                            },
                                            name: "size".to_string(),
                                            value: Value::Integer(32)
                                        },
                                        FieldOption {
                                            position: FilePosition {
                                                line: 54,
                                                column: 36
                                            },
                                            name: "range".to_string(),
                                            value: Value::Range(Some(0), None)
                                        }
                                    ]
                                },
                                Field {
                                    name: "limit".to_string(),
                                    position: FilePosition {
                                        line: 55,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Integer".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: true,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 55,
                                            column: 26
                                        },
                                        name: "range".to_string(),
                                        value: Value::Range(Some(1), Some(200))
                                    }]
                                }
                            ]
                        }),
                        NamespacePart::Struct(Struct {
                            name: "UserList".to_string(),
                            position: FilePosition {
                                line: 58,
                                column: 5
                            },
                            generics: vec![],
                            fields: vec![
                                Field {
                                    name: "count".to_string(),
                                    position: FilePosition {
                                        line: 59,
                                        column: 9
                                    },
                                    type_: Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Integer".to_string(),
                                        generics: vec![]
                                    }),
                                    optional: false,
                                    options: vec![
                                        FieldOption {
                                            position: FilePosition {
                                                line: 60,
                                                column: 13
                                            },
                                            name: "range".to_string(),
                                            value: Value::Range(Some(0), Some(65535))
                                        },
                                        FieldOption {
                                            position: FilePosition {
                                                line: 61,
                                                column: 13
                                            },
                                            name: "help".to_string(),
                                            value: Value::String(
                                                "Count of objects returned".to_string()
                                            )
                                        }
                                    ]
                                },
                                Field {
                                    name: "users".to_string(),
                                    position: FilePosition {
                                        line: 63,
                                        column: 9
                                    },
                                    type_: Type::Array(Box::new(Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "UserRead".to_string(),
                                        generics: vec![],
                                    }))),
                                    optional: false,
                                    options: vec![FieldOption {
                                        position: FilePosition {
                                            line: 63,
                                            column: 28
                                        },
                                        name: "length".to_string(),
                                        value: Value::Range(Some(0), Some(128))
                                    }]
                                },
                                Field {
                                    name: "permissions".to_string(),
                                    position: FilePosition {
                                        line: 64,
                                        column: 9
                                    },
                                    type_: Type::Map(
                                        Box::new(Type::Ref(TypeRef {
                                            abs: false,
                                            ns: vec![],
                                            name: "UUID".to_string(),
                                            generics: vec![]
                                        })),
                                        Box::new(Type::Ref(TypeRef {
                                            abs: false,
                                            ns: vec![],
                                            name: "String".to_string(),
                                            generics: vec![]
                                        })),
                                    ),
                                    optional: false,
                                    options: vec![]
                                }
                            ]
                        }),
                        NamespacePart::Enum(Enum {
                            name: "GetError".to_string(),
                            generics: vec![],
                            position: FilePosition {
                                line: 67,
                                column: 5
                            },
                            extends: None,
                            variants: vec![
                                EnumVariant {
                                    name: "PermissionDenied".to_string(),
                                    value_type: None,
                                },
                                EnumVariant {
                                    name: "DoesNotExist".to_string(),
                                    value_type: None,
                                },
                            ],
                        }),
                        NamespacePart::Enum(Enum {
                            name: "ListError".to_string(),
                            generics: vec![],
                            position: FilePosition {
                                line: 72,
                                column: 5
                            },
                            extends: None,
                            variants: vec![EnumVariant {
                                name: "PermissionDenied".to_string(),
                                value_type: None,
                            },],
                        }),
                        NamespacePart::Service(Service {
                            name: "ExampleService".to_string(),
                            position: FilePosition {
                                line: 76,
                                column: 5
                            },
                            methods: vec![
                                Method {
                                    name: "get_version".to_string(),
                                    input: None,
                                    output: Some(Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "String".to_string(),
                                        generics: vec![]
                                    })),
                                },
                                Method {
                                    name: "user_get".to_string(),
                                    input: Some(Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "UserRequest".to_string(),
                                        generics: vec![],
                                    })),
                                    output: Some(Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Result".to_string(),
                                        generics: vec![
                                            Type::Ref(TypeRef {
                                                abs: false,
                                                ns: vec![],
                                                name: "UserRead".to_string(),
                                                generics: vec![]
                                            }),
                                            Type::Ref(TypeRef {
                                                abs: false,
                                                ns: vec![],
                                                name: "GetError".to_string(),
                                                generics: vec![],
                                            })
                                        ]
                                    }))
                                },
                                Method {
                                    name: "user_list".to_string(),
                                    input: Some(Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "UserListRequest".to_string(),
                                        generics: vec![],
                                    })),
                                    output: Some(Type::Ref(TypeRef {
                                        abs: false,
                                        ns: vec![],
                                        name: "Result".to_string(),
                                        generics: vec![
                                            Type::Ref(TypeRef {
                                                abs: false,
                                                ns: vec![],
                                                name: "UserList".to_string(),
                                                generics: vec![],
                                            }),
                                            Type::Ref(TypeRef {
                                                abs: false,
                                                ns: vec![],
                                                name: "ListError".to_string(),
                                                generics: vec![],
                                            })
                                        ]
                                    }))
                                },
                            ]
                        }),
                    ]
                }),]
            }
        },
    );
}
