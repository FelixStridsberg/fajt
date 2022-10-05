### Source
```js parse:expr check-format:no
{
    await: true,
    break: true,
    case: true,
    catch: true,
    class: true,
    const: true,
    continue: true,
    debugger: true,
    default: true,
    delete: true,
    do: true,
    else: true,
    enum: true,
    export: true,
    extends: true,
    false: true,
    finally: true,
    for: true,
    function: true,
    if: true,
    import: true,
    in: true,
    instanceof: true,
    new: true,
    null: true,
    return: true,
    super: true,
    switch: true,
    this: true,
    throw: true,
    true: true,
    try: true,
    typeof: true,
    var: true,
    void: true,
    while: true,
    with: true,
    yield: true,
}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:650",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "6:17",
              "name": {
                "Ident": {
                  "span": "6:11",
                  "name": "await"
                }
              },
              "value": {
                "Literal": {
                  "span": "13:17",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "23:34",
              "name": {
                "Ident": {
                  "span": "23:28",
                  "name": "break"
                }
              },
              "value": {
                "Literal": {
                  "span": "30:34",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "40:50",
              "name": {
                "Ident": {
                  "span": "40:44",
                  "name": "case"
                }
              },
              "value": {
                "Literal": {
                  "span": "46:50",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "56:67",
              "name": {
                "Ident": {
                  "span": "56:61",
                  "name": "catch"
                }
              },
              "value": {
                "Literal": {
                  "span": "63:67",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "73:84",
              "name": {
                "Ident": {
                  "span": "73:78",
                  "name": "class"
                }
              },
              "value": {
                "Literal": {
                  "span": "80:84",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "90:101",
              "name": {
                "Ident": {
                  "span": "90:95",
                  "name": "const"
                }
              },
              "value": {
                "Literal": {
                  "span": "97:101",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "107:121",
              "name": {
                "Ident": {
                  "span": "107:115",
                  "name": "continue"
                }
              },
              "value": {
                "Literal": {
                  "span": "117:121",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "127:141",
              "name": {
                "Ident": {
                  "span": "127:135",
                  "name": "debugger"
                }
              },
              "value": {
                "Literal": {
                  "span": "137:141",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "147:160",
              "name": {
                "Ident": {
                  "span": "147:154",
                  "name": "default"
                }
              },
              "value": {
                "Literal": {
                  "span": "156:160",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "166:178",
              "name": {
                "Ident": {
                  "span": "166:172",
                  "name": "delete"
                }
              },
              "value": {
                "Literal": {
                  "span": "174:178",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "184:192",
              "name": {
                "Ident": {
                  "span": "184:186",
                  "name": "do"
                }
              },
              "value": {
                "Literal": {
                  "span": "188:192",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "198:208",
              "name": {
                "Ident": {
                  "span": "198:202",
                  "name": "else"
                }
              },
              "value": {
                "Literal": {
                  "span": "204:208",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "214:224",
              "name": {
                "Ident": {
                  "span": "214:218",
                  "name": "enum"
                }
              },
              "value": {
                "Literal": {
                  "span": "220:224",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "230:242",
              "name": {
                "Ident": {
                  "span": "230:236",
                  "name": "export"
                }
              },
              "value": {
                "Literal": {
                  "span": "238:242",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "248:261",
              "name": {
                "Ident": {
                  "span": "248:255",
                  "name": "extends"
                }
              },
              "value": {
                "Literal": {
                  "span": "257:261",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "267:278",
              "name": {
                "Ident": {
                  "span": "267:272",
                  "name": "false"
                }
              },
              "value": {
                "Literal": {
                  "span": "274:278",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "284:297",
              "name": {
                "Ident": {
                  "span": "284:291",
                  "name": "finally"
                }
              },
              "value": {
                "Literal": {
                  "span": "293:297",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "303:312",
              "name": {
                "Ident": {
                  "span": "303:306",
                  "name": "for"
                }
              },
              "value": {
                "Literal": {
                  "span": "308:312",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "318:332",
              "name": {
                "Ident": {
                  "span": "318:326",
                  "name": "function"
                }
              },
              "value": {
                "Literal": {
                  "span": "328:332",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "338:346",
              "name": {
                "Ident": {
                  "span": "338:340",
                  "name": "if"
                }
              },
              "value": {
                "Literal": {
                  "span": "342:346",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "352:364",
              "name": {
                "Ident": {
                  "span": "352:358",
                  "name": "import"
                }
              },
              "value": {
                "Literal": {
                  "span": "360:364",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "370:378",
              "name": {
                "Ident": {
                  "span": "370:372",
                  "name": "in"
                }
              },
              "value": {
                "Literal": {
                  "span": "374:378",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "384:400",
              "name": {
                "Ident": {
                  "span": "384:394",
                  "name": "instanceof"
                }
              },
              "value": {
                "Literal": {
                  "span": "396:400",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "406:415",
              "name": {
                "Ident": {
                  "span": "406:409",
                  "name": "new"
                }
              },
              "value": {
                "Literal": {
                  "span": "411:415",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "421:431",
              "name": {
                "Ident": {
                  "span": "421:425",
                  "name": "null"
                }
              },
              "value": {
                "Literal": {
                  "span": "427:431",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "437:449",
              "name": {
                "Ident": {
                  "span": "437:443",
                  "name": "return"
                }
              },
              "value": {
                "Literal": {
                  "span": "445:449",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "455:466",
              "name": {
                "Ident": {
                  "span": "455:460",
                  "name": "super"
                }
              },
              "value": {
                "Literal": {
                  "span": "462:466",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "472:484",
              "name": {
                "Ident": {
                  "span": "472:478",
                  "name": "switch"
                }
              },
              "value": {
                "Literal": {
                  "span": "480:484",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "490:500",
              "name": {
                "Ident": {
                  "span": "490:494",
                  "name": "this"
                }
              },
              "value": {
                "Literal": {
                  "span": "496:500",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "506:517",
              "name": {
                "Ident": {
                  "span": "506:511",
                  "name": "throw"
                }
              },
              "value": {
                "Literal": {
                  "span": "513:517",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "523:533",
              "name": {
                "Ident": {
                  "span": "523:527",
                  "name": "true"
                }
              },
              "value": {
                "Literal": {
                  "span": "529:533",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "539:548",
              "name": {
                "Ident": {
                  "span": "539:542",
                  "name": "try"
                }
              },
              "value": {
                "Literal": {
                  "span": "544:548",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "554:566",
              "name": {
                "Ident": {
                  "span": "554:560",
                  "name": "typeof"
                }
              },
              "value": {
                "Literal": {
                  "span": "562:566",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "572:581",
              "name": {
                "Ident": {
                  "span": "572:575",
                  "name": "var"
                }
              },
              "value": {
                "Literal": {
                  "span": "577:581",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "587:597",
              "name": {
                "Ident": {
                  "span": "587:591",
                  "name": "void"
                }
              },
              "value": {
                "Literal": {
                  "span": "593:597",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "603:614",
              "name": {
                "Ident": {
                  "span": "603:608",
                  "name": "while"
                }
              },
              "value": {
                "Literal": {
                  "span": "610:614",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "620:630",
              "name": {
                "Ident": {
                  "span": "620:624",
                  "name": "with"
                }
              },
              "value": {
                "Literal": {
                  "span": "626:630",
                  "literal": {
                    "Boolean": true
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "636:647",
              "name": {
                "Ident": {
                  "span": "636:641",
                  "name": "yield"
                }
              },
              "value": {
                "Literal": {
                  "span": "643:647",
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
