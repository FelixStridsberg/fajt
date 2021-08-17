```js
export {a}
export {a};
export {a} from 'b'
export {a} from 'b';
export * from 'b'
export * from 'b';
export * as b from 'c'
export * as b from 'c';
export var a = b
export var a = b;
export let a = b
export let a = b;
export const a = b
export const a = b;
export async function name() {}
export async function name() {};
export function name() {}
export function name() {};
export class name {}
export class name {};
```

```json
{
  "Module": {
    "span": "0:417",
    "body": [
      {
        "ExportDeclaration": {
          "Named": {
            "span": "0:10",
            "named_exports": [
              {
                "span": "8:9",
                "name": {
                  "span": "8:9",
                  "name": "a"
                },
                "alias_of": null
              }
            ],
            "from": null
          }
        }
      },
      {
        "ExportDeclaration": {
          "Named": {
            "span": "11:22",
            "named_exports": [
              {
                "span": "19:20",
                "name": {
                  "span": "19:20",
                  "name": "a"
                },
                "alias_of": null
              }
            ],
            "from": null
          }
        }
      },
      {
        "ExportDeclaration": {
          "Named": {
            "span": "23:42",
            "named_exports": [
              {
                "span": "31:32",
                "name": {
                  "span": "31:32",
                  "name": "a"
                },
                "alias_of": null
              }
            ],
            "from": "b"
          }
        }
      },
      {
        "ExportDeclaration": {
          "Named": {
            "span": "43:63",
            "named_exports": [
              {
                "span": "51:52",
                "name": {
                  "span": "51:52",
                  "name": "a"
                },
                "alias_of": null
              }
            ],
            "from": "b"
          }
        }
      },
      {
        "ExportDeclaration": {
          "Namespace": {
            "span": "64:81",
            "alias": null,
            "from": "b"
          }
        }
      },
      {
        "ExportDeclaration": {
          "Namespace": {
            "span": "82:100",
            "alias": null,
            "from": "b"
          }
        }
      },
      {
        "ExportDeclaration": {
          "Namespace": {
            "span": "101:123",
            "alias": {
              "span": "113:114",
              "name": "b"
            },
            "from": "c"
          }
        }
      },
      {
        "ExportDeclaration": {
          "Namespace": {
            "span": "124:147",
            "alias": {
              "span": "136:137",
              "name": "b"
            },
            "from": "c"
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "148:164",
            "decl": {
              "Variable": {
                "span": "155:164",
                "kind": "Var",
                "declarations": [
                  {
                    "span": "159:164",
                    "pattern": {
                      "Ident": {
                        "span": "159:160",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "163:164",
                        "name": "b"
                      }
                    }
                  }
                ]
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "165:182",
            "decl": {
              "Variable": {
                "span": "172:182",
                "kind": "Var",
                "declarations": [
                  {
                    "span": "176:181",
                    "pattern": {
                      "Ident": {
                        "span": "176:177",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "180:181",
                        "name": "b"
                      }
                    }
                  }
                ]
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "183:199",
            "decl": {
              "Variable": {
                "span": "190:199",
                "kind": "Let",
                "declarations": [
                  {
                    "span": "194:199",
                    "pattern": {
                      "Ident": {
                        "span": "194:195",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "198:199",
                        "name": "b"
                      }
                    }
                  }
                ]
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "200:217",
            "decl": {
              "Variable": {
                "span": "207:217",
                "kind": "Let",
                "declarations": [
                  {
                    "span": "211:216",
                    "pattern": {
                      "Ident": {
                        "span": "211:212",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "215:216",
                        "name": "b"
                      }
                    }
                  }
                ]
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "218:236",
            "decl": {
              "Variable": {
                "span": "225:236",
                "kind": "Const",
                "declarations": [
                  {
                    "span": "231:236",
                    "pattern": {
                      "Ident": {
                        "span": "231:232",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "235:236",
                        "name": "b"
                      }
                    }
                  }
                ]
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "237:256",
            "decl": {
              "Variable": {
                "span": "244:256",
                "kind": "Const",
                "declarations": [
                  {
                    "span": "250:255",
                    "pattern": {
                      "Ident": {
                        "span": "250:251",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "254:255",
                        "name": "b"
                      }
                    }
                  }
                ]
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "257:288",
            "decl": {
              "FunctionDecl": {
                "span": "264:288",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "279:283",
                  "name": "name"
                },
                "parameters": {
                  "span": "283:285",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "286:288",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "289:320",
            "decl": {
              "FunctionDecl": {
                "span": "296:320",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "311:315",
                  "name": "name"
                },
                "parameters": {
                  "span": "315:317",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "318:320",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "320:321"
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "322:347",
            "decl": {
              "FunctionDecl": {
                "span": "329:347",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "338:342",
                  "name": "name"
                },
                "parameters": {
                  "span": "342:344",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "345:347",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "348:373",
            "decl": {
              "FunctionDecl": {
                "span": "355:373",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "364:368",
                  "name": "name"
                },
                "parameters": {
                  "span": "368:370",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "371:373",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "373:374"
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "375:395",
            "decl": {
              "Class": {
                "span": "382:395",
                "identifier": {
                  "span": "388:392",
                  "name": "name"
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "396:416",
            "decl": {
              "Class": {
                "span": "403:416",
                "identifier": {
                  "span": "409:413",
                  "name": "name"
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "416:417"
        }
      }
    ]
  }
}
```
