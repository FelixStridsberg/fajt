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
      "Expr": {
        "AssignmentPattern": {
          "Object": {
            "span": "0:548",
            "props": [
              {
                "Named": {
                  "span": "6:14",
                  "name": {
                    "Ident": {
                      "span": "6:11",
                      "name": "await"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "13:14",
                      "name": "a"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "20:28",
                  "name": {
                    "Ident": {
                      "span": "20:25",
                      "name": "break"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "27:28",
                      "name": "b"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "34:41",
                  "name": {
                    "Ident": {
                      "span": "34:38",
                      "name": "case"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "40:41",
                      "name": "c"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "47:55",
                  "name": {
                    "Ident": {
                      "span": "47:52",
                      "name": "catch"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "54:55",
                      "name": "d"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "61:69",
                  "name": {
                    "Ident": {
                      "span": "61:66",
                      "name": "class"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "68:69",
                      "name": "e"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "75:83",
                  "name": {
                    "Ident": {
                      "span": "75:80",
                      "name": "const"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "82:83",
                      "name": "f"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "89:100",
                  "name": {
                    "Ident": {
                      "span": "89:97",
                      "name": "continue"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "99:100",
                      "name": "g"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "106:117",
                  "name": {
                    "Ident": {
                      "span": "106:114",
                      "name": "debugger"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "116:117",
                      "name": "h"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "123:133",
                  "name": {
                    "Ident": {
                      "span": "123:130",
                      "name": "default"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "132:133",
                      "name": "i"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "139:148",
                  "name": {
                    "Ident": {
                      "span": "139:145",
                      "name": "delete"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "147:148",
                      "name": "j"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "154:159",
                  "name": {
                    "Ident": {
                      "span": "154:156",
                      "name": "do"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "158:159",
                      "name": "k"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "165:172",
                  "name": {
                    "Ident": {
                      "span": "165:169",
                      "name": "else"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "171:172",
                      "name": "l"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "178:185",
                  "name": {
                    "Ident": {
                      "span": "178:182",
                      "name": "enum"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "184:185",
                      "name": "m"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "191:200",
                  "name": {
                    "Ident": {
                      "span": "191:197",
                      "name": "export"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "199:200",
                      "name": "n"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "206:216",
                  "name": {
                    "Ident": {
                      "span": "206:213",
                      "name": "extends"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "215:216",
                      "name": "o"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "222:230",
                  "name": {
                    "Ident": {
                      "span": "222:227",
                      "name": "false"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "229:230",
                      "name": "p"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "236:246",
                  "name": {
                    "Ident": {
                      "span": "236:243",
                      "name": "finally"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "245:246",
                      "name": "q"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "252:258",
                  "name": {
                    "Ident": {
                      "span": "252:255",
                      "name": "for"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "257:258",
                      "name": "r"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "264:275",
                  "name": {
                    "Ident": {
                      "span": "264:272",
                      "name": "function"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "274:275",
                      "name": "s"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "281:286",
                  "name": {
                    "Ident": {
                      "span": "281:283",
                      "name": "if"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "285:286",
                      "name": "t"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "292:301",
                  "name": {
                    "Ident": {
                      "span": "292:298",
                      "name": "import"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "300:301",
                      "name": "u"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "307:312",
                  "name": {
                    "Ident": {
                      "span": "307:309",
                      "name": "in"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "311:312",
                      "name": "v"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "318:331",
                  "name": {
                    "Ident": {
                      "span": "318:328",
                      "name": "instanceof"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "330:331",
                      "name": "w"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "337:343",
                  "name": {
                    "Ident": {
                      "span": "337:340",
                      "name": "new"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "342:343",
                      "name": "x"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "349:356",
                  "name": {
                    "Ident": {
                      "span": "349:353",
                      "name": "null"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "355:356",
                      "name": "y"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "362:371",
                  "name": {
                    "Ident": {
                      "span": "362:368",
                      "name": "return"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "370:371",
                      "name": "z"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "377:386",
                  "name": {
                    "Ident": {
                      "span": "377:382",
                      "name": "super"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "384:386",
                      "name": "aa"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "392:402",
                  "name": {
                    "Ident": {
                      "span": "392:398",
                      "name": "switch"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "400:402",
                      "name": "ab"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "408:416",
                  "name": {
                    "Ident": {
                      "span": "408:412",
                      "name": "this"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "414:416",
                      "name": "ac"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "422:431",
                  "name": {
                    "Ident": {
                      "span": "422:427",
                      "name": "throw"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "429:431",
                      "name": "ad"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "437:445",
                  "name": {
                    "Ident": {
                      "span": "437:441",
                      "name": "true"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "443:445",
                      "name": "ae"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "451:458",
                  "name": {
                    "Ident": {
                      "span": "451:454",
                      "name": "try"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "456:458",
                      "name": "af"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "464:474",
                  "name": {
                    "Ident": {
                      "span": "464:470",
                      "name": "typeof"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "472:474",
                      "name": "ag"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "480:487",
                  "name": {
                    "Ident": {
                      "span": "480:483",
                      "name": "var"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "485:487",
                      "name": "ah"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "493:501",
                  "name": {
                    "Ident": {
                      "span": "493:497",
                      "name": "void"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "499:501",
                      "name": "ai"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "507:516",
                  "name": {
                    "Ident": {
                      "span": "507:512",
                      "name": "while"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "514:516",
                      "name": "aj"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "522:530",
                  "name": {
                    "Ident": {
                      "span": "522:526",
                      "name": "with"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "528:530",
                      "name": "ak"
                    }
                  },
                  "initializer": null
                }
              },
              {
                "Named": {
                  "span": "536:545",
                  "name": {
                    "Ident": {
                      "span": "536:541",
                      "name": "yield"
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "543:545",
                      "name": "al"
                    }
                  },
                  "initializer": null
                }
              }
            ],
            "rest": null
          }
        }
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
