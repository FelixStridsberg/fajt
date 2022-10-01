### Source
```js parse:expr check-format:no
{
    await: a,
    break: b,
    case: c,
    catch: d,
    class: e,
    const: f,
    continue: g,
    debugger: h,
    default: i,
    delete: j,
    do: k,
    else: l,
    enum: m,
    export: n,
    extends: o,
    false: p,
    finally: q,
    for: r,
    function: s,
    if: t,
    import: u,
    in: v,
    instanceof: w,
    new: x,
    null: y,
    return: z,
    super: aa,
    switch: ab,
    this: ac,
    throw: ad,
    true: ae,
    try: af,
    typeof: ag,
    var: ah,
    void: ai,
    while: aj,
    with: ak,
    yield: al,
} = {}
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:553",
    "operator": "Assign",
    "left": {
      "ObjectBinding": {
        "span": "0:548",
        "props": [
          {
            "Named": {
              "span": "6:14",
              "property": {
                "Ident": {
                  "span": "6:11",
                  "name": "await"
                }
              },
              "binding": {
                "span": "13:14",
                "pattern": {
                  "Ident": {
                    "span": "13:14",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "20:28",
              "property": {
                "Ident": {
                  "span": "20:25",
                  "name": "break"
                }
              },
              "binding": {
                "span": "27:28",
                "pattern": {
                  "Ident": {
                    "span": "27:28",
                    "name": "b"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "34:41",
              "property": {
                "Ident": {
                  "span": "34:38",
                  "name": "case"
                }
              },
              "binding": {
                "span": "40:41",
                "pattern": {
                  "Ident": {
                    "span": "40:41",
                    "name": "c"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "47:55",
              "property": {
                "Ident": {
                  "span": "47:52",
                  "name": "catch"
                }
              },
              "binding": {
                "span": "54:55",
                "pattern": {
                  "Ident": {
                    "span": "54:55",
                    "name": "d"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "61:69",
              "property": {
                "Ident": {
                  "span": "61:66",
                  "name": "class"
                }
              },
              "binding": {
                "span": "68:69",
                "pattern": {
                  "Ident": {
                    "span": "68:69",
                    "name": "e"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "75:83",
              "property": {
                "Ident": {
                  "span": "75:80",
                  "name": "const"
                }
              },
              "binding": {
                "span": "82:83",
                "pattern": {
                  "Ident": {
                    "span": "82:83",
                    "name": "f"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "89:100",
              "property": {
                "Ident": {
                  "span": "89:97",
                  "name": "continue"
                }
              },
              "binding": {
                "span": "99:100",
                "pattern": {
                  "Ident": {
                    "span": "99:100",
                    "name": "g"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "106:117",
              "property": {
                "Ident": {
                  "span": "106:114",
                  "name": "debugger"
                }
              },
              "binding": {
                "span": "116:117",
                "pattern": {
                  "Ident": {
                    "span": "116:117",
                    "name": "h"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "123:133",
              "property": {
                "Ident": {
                  "span": "123:130",
                  "name": "default"
                }
              },
              "binding": {
                "span": "132:133",
                "pattern": {
                  "Ident": {
                    "span": "132:133",
                    "name": "i"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "139:148",
              "property": {
                "Ident": {
                  "span": "139:145",
                  "name": "delete"
                }
              },
              "binding": {
                "span": "147:148",
                "pattern": {
                  "Ident": {
                    "span": "147:148",
                    "name": "j"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "154:159",
              "property": {
                "Ident": {
                  "span": "154:156",
                  "name": "do"
                }
              },
              "binding": {
                "span": "158:159",
                "pattern": {
                  "Ident": {
                    "span": "158:159",
                    "name": "k"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "165:172",
              "property": {
                "Ident": {
                  "span": "165:169",
                  "name": "else"
                }
              },
              "binding": {
                "span": "171:172",
                "pattern": {
                  "Ident": {
                    "span": "171:172",
                    "name": "l"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "178:185",
              "property": {
                "Ident": {
                  "span": "178:182",
                  "name": "enum"
                }
              },
              "binding": {
                "span": "184:185",
                "pattern": {
                  "Ident": {
                    "span": "184:185",
                    "name": "m"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "191:200",
              "property": {
                "Ident": {
                  "span": "191:197",
                  "name": "export"
                }
              },
              "binding": {
                "span": "199:200",
                "pattern": {
                  "Ident": {
                    "span": "199:200",
                    "name": "n"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "206:216",
              "property": {
                "Ident": {
                  "span": "206:213",
                  "name": "extends"
                }
              },
              "binding": {
                "span": "215:216",
                "pattern": {
                  "Ident": {
                    "span": "215:216",
                    "name": "o"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "222:230",
              "property": {
                "Ident": {
                  "span": "222:227",
                  "name": "false"
                }
              },
              "binding": {
                "span": "229:230",
                "pattern": {
                  "Ident": {
                    "span": "229:230",
                    "name": "p"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "236:246",
              "property": {
                "Ident": {
                  "span": "236:243",
                  "name": "finally"
                }
              },
              "binding": {
                "span": "245:246",
                "pattern": {
                  "Ident": {
                    "span": "245:246",
                    "name": "q"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "252:258",
              "property": {
                "Ident": {
                  "span": "252:255",
                  "name": "for"
                }
              },
              "binding": {
                "span": "257:258",
                "pattern": {
                  "Ident": {
                    "span": "257:258",
                    "name": "r"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "264:275",
              "property": {
                "Ident": {
                  "span": "264:272",
                  "name": "function"
                }
              },
              "binding": {
                "span": "274:275",
                "pattern": {
                  "Ident": {
                    "span": "274:275",
                    "name": "s"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "281:286",
              "property": {
                "Ident": {
                  "span": "281:283",
                  "name": "if"
                }
              },
              "binding": {
                "span": "285:286",
                "pattern": {
                  "Ident": {
                    "span": "285:286",
                    "name": "t"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "292:301",
              "property": {
                "Ident": {
                  "span": "292:298",
                  "name": "import"
                }
              },
              "binding": {
                "span": "300:301",
                "pattern": {
                  "Ident": {
                    "span": "300:301",
                    "name": "u"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "307:312",
              "property": {
                "Ident": {
                  "span": "307:309",
                  "name": "in"
                }
              },
              "binding": {
                "span": "311:312",
                "pattern": {
                  "Ident": {
                    "span": "311:312",
                    "name": "v"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "318:331",
              "property": {
                "Ident": {
                  "span": "318:328",
                  "name": "instanceof"
                }
              },
              "binding": {
                "span": "330:331",
                "pattern": {
                  "Ident": {
                    "span": "330:331",
                    "name": "w"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "337:343",
              "property": {
                "Ident": {
                  "span": "337:340",
                  "name": "new"
                }
              },
              "binding": {
                "span": "342:343",
                "pattern": {
                  "Ident": {
                    "span": "342:343",
                    "name": "x"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "349:356",
              "property": {
                "Ident": {
                  "span": "349:353",
                  "name": "null"
                }
              },
              "binding": {
                "span": "355:356",
                "pattern": {
                  "Ident": {
                    "span": "355:356",
                    "name": "y"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "362:371",
              "property": {
                "Ident": {
                  "span": "362:368",
                  "name": "return"
                }
              },
              "binding": {
                "span": "370:371",
                "pattern": {
                  "Ident": {
                    "span": "370:371",
                    "name": "z"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "377:386",
              "property": {
                "Ident": {
                  "span": "377:382",
                  "name": "super"
                }
              },
              "binding": {
                "span": "384:386",
                "pattern": {
                  "Ident": {
                    "span": "384:386",
                    "name": "aa"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "392:402",
              "property": {
                "Ident": {
                  "span": "392:398",
                  "name": "switch"
                }
              },
              "binding": {
                "span": "400:402",
                "pattern": {
                  "Ident": {
                    "span": "400:402",
                    "name": "ab"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "408:416",
              "property": {
                "Ident": {
                  "span": "408:412",
                  "name": "this"
                }
              },
              "binding": {
                "span": "414:416",
                "pattern": {
                  "Ident": {
                    "span": "414:416",
                    "name": "ac"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "422:431",
              "property": {
                "Ident": {
                  "span": "422:427",
                  "name": "throw"
                }
              },
              "binding": {
                "span": "429:431",
                "pattern": {
                  "Ident": {
                    "span": "429:431",
                    "name": "ad"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "437:445",
              "property": {
                "Ident": {
                  "span": "437:441",
                  "name": "true"
                }
              },
              "binding": {
                "span": "443:445",
                "pattern": {
                  "Ident": {
                    "span": "443:445",
                    "name": "ae"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "451:458",
              "property": {
                "Ident": {
                  "span": "451:454",
                  "name": "try"
                }
              },
              "binding": {
                "span": "456:458",
                "pattern": {
                  "Ident": {
                    "span": "456:458",
                    "name": "af"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "464:474",
              "property": {
                "Ident": {
                  "span": "464:470",
                  "name": "typeof"
                }
              },
              "binding": {
                "span": "472:474",
                "pattern": {
                  "Ident": {
                    "span": "472:474",
                    "name": "ag"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "480:487",
              "property": {
                "Ident": {
                  "span": "480:483",
                  "name": "var"
                }
              },
              "binding": {
                "span": "485:487",
                "pattern": {
                  "Ident": {
                    "span": "485:487",
                    "name": "ah"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "493:501",
              "property": {
                "Ident": {
                  "span": "493:497",
                  "name": "void"
                }
              },
              "binding": {
                "span": "499:501",
                "pattern": {
                  "Ident": {
                    "span": "499:501",
                    "name": "ai"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "507:516",
              "property": {
                "Ident": {
                  "span": "507:512",
                  "name": "while"
                }
              },
              "binding": {
                "span": "514:516",
                "pattern": {
                  "Ident": {
                    "span": "514:516",
                    "name": "aj"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "522:530",
              "property": {
                "Ident": {
                  "span": "522:526",
                  "name": "with"
                }
              },
              "binding": {
                "span": "528:530",
                "pattern": {
                  "Ident": {
                    "span": "528:530",
                    "name": "ak"
                  }
                },
                "initializer": null
              }
            }
          },
          {
            "Named": {
              "span": "536:545",
              "property": {
                "Ident": {
                  "span": "536:541",
                  "name": "yield"
                }
              },
              "binding": {
                "span": "543:545",
                "pattern": {
                  "Ident": {
                    "span": "543:545",
                    "name": "al"
                  }
                },
                "initializer": null
              }
            }
          }
        ],
        "rest": null
      }
    },
    "right": {
      "Literal": {
        "span": "551:553",
        "literal": {
          "Object": {
            "props": []
          }
        }
      }
    }
  }
}
```
