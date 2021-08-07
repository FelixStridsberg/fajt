#[test]
fn new() {
    parser_test_json!(
        input: "new a",
        expr_output: r#"
            {
              "New": {
                "span": "0:5",
                "callee": {
                  "IdentRef": {
                    "span": "4:5",
                    "name": "a"
                  }
                },
                "arguments_span": null,
                "arguments": []
              }
            }
        "#
    );
}

#[test]
fn new_nested() {
    parser_test_json!(
        input: "new new a",
        expr_output: r#"
            {
              "New": {
                "span": "0:9",
                "callee": {
                  "New": {
                    "span": "4:9",
                    "callee": {
                      "IdentRef": {
                        "span": "8:9",
                        "name": "a"
                      }
                    },
                    "arguments_span": null,
                    "arguments": []
                  }
                },
                "arguments_span": null,
                "arguments": []
              }
            }
        "#
    );
}

#[test]
fn new_empty_arguments() {
    parser_test_json!(
        input: "new a()",
        expr_output: r#"
            {
              "New": {
                "span": "0:7",
                "callee": {
                  "IdentRef": {
                    "span": "4:5",
                    "name": "a"
                  }
                },
                "arguments_span": "5:7",
                "arguments": []
              }
            }
        "#
    );
}

#[test]
fn new_empty_arguments_member() {
    parser_test_json!(
        input: "new a.b()",
        expr_output: r#"
            {
              "New": {
                "span": "0:9",
                "callee": {
                  "Member": {
                    "span": "4:7",
                    "object": {
                      "Expr": {
                        "IdentRef": {
                          "span": "4:5",
                          "name": "a"
                        }
                      }
                    },
                    "property": {
                      "Ident": {
                        "span": "6:7",
                        "name": "b"
                      }
                    }
                  }
                },
                "arguments_span": "7:9",
                "arguments": []
              }
            }
        "#
    );
}

#[test]
fn new_with_arguments() {
    parser_test_json!(
        input: "new a(b, !null)",
        expr_output: r#"
            {
              "New": {
                "span": "0:15",
                "callee": {
                  "IdentRef": {
                    "span": "4:5",
                    "name": "a"
                  }
                },
                "arguments_span": "5:15",
                "arguments": [
                  {
                    "Expr": {
                      "IdentRef": {
                        "span": "6:7",
                        "name": "b"
                      }
                    }
                  },
                  {
                    "Expr": {
                      "Unary": {
                        "span": "9:14",
                        "operator": "Not",
                        "argument": {
                          "Literal": {
                            "span": "10:14",
                            "literal": "Null"
                          }
                        }
                      }
                    }
                  }
                ]
              }
            }
        "#
    );
}

#[test]
fn new_with_spread_arguments() {
    parser_test_json!(
        input: "new a(...b, c, ...[])",
        expr_output: r#"
            {
              "New": {
                "span": "0:21",
                "callee": {
                  "IdentRef": {
                    "span": "4:5",
                    "name": "a"
                  }
                },
                "arguments_span": "5:21",
                "arguments": [
                  {
                    "Spread": {
                      "IdentRef": {
                        "span": "9:10",
                        "name": "b"
                      }
                    }
                  },
                  {
                    "Expr": {
                      "IdentRef": {
                        "span": "12:13",
                        "name": "c"
                      }
                    }
                  },
                  {
                    "Spread": {
                      "Literal": {
                        "span": "18:20",
                        "literal": {
                          "Array": {
                            "elements": []
                          }
                        }
                      }
                    }
                  }
                ]
              }
            }
        "#
    );
}
