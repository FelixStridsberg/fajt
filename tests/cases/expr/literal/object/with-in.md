### Source
```js parse:expr check-format:no
{
    a: "b" in c,
    ...("d" in e ? {} : {}),
    ["f" in g]: true
}
```

### Output: minified
```js
{a:"b"in c,...("d"in e?{}:{}),["f"in g]:true}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:70",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "6:17",
              "name": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "value": {
                "Binary": {
                  "span": "9:17",
                  "operator": "In",
                  "left": {
                    "Literal": {
                      "span": "9:12",
                      "literal": {
                        "String": {
                          "value": "b",
                          "delimiter": "\""
                        }
                      }
                    }
                  },
                  "right": {
                    "IdentRef": {
                      "span": "16:17",
                      "name": "c"
                    }
                  }
                }
              }
            }
          },
          {
            "Spread": {
              "Parenthesized": {
                "span": "26:46",
                "expression": {
                  "Conditional": {
                    "span": "27:45",
                    "condition": {
                      "Binary": {
                        "span": "27:35",
                        "operator": "In",
                        "left": {
                          "Literal": {
                            "span": "27:30",
                            "literal": {
                              "String": {
                                "value": "d",
                                "delimiter": "\""
                              }
                            }
                          }
                        },
                        "right": {
                          "IdentRef": {
                            "span": "34:35",
                            "name": "e"
                          }
                        }
                      }
                    },
                    "consequent": {
                      "Literal": {
                        "span": "38:40",
                        "literal": {
                          "Object": {
                            "props": []
                          }
                        }
                      }
                    },
                    "alternate": {
                      "Literal": {
                        "span": "43:45",
                        "literal": {
                          "Object": {
                            "props": []
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "52:68",
              "name": {
                "Computed": {
                  "Binary": {
                    "span": "53:61",
                    "operator": "In",
                    "left": {
                      "Literal": {
                        "span": "53:56",
                        "literal": {
                          "String": {
                            "value": "f",
                            "delimiter": "\""
                          }
                        }
                      }
                    },
                    "right": {
                      "IdentRef": {
                        "span": "60:61",
                        "name": "g"
                      }
                    }
                  }
                }
              },
              "value": {
                "Literal": {
                  "span": "64:68",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          }
        ]
      }
    }
  }
}
```
