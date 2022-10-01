### Source
```js parse:expr check-format:no
{
    await() {},
    break() {},
    case() {},
    catch() {},
    class() {},
    const() {},
    continue() {},
    debugger() {},
    default() {},
    delete() {},
    do() {},
    else() {},
    enum() {},
    export() {},
    extends() {},
    false() {},
    finally() {},
    for() {},
    function() {},
    if() {},
    import() {},
    in() {},
    instanceof() {},
    new() {},
    null() {},
    return() {},
    super() {},
    switch() {},
    this() {},
    throw() {},
    true() {},
    try() {},
    typeof() {},
    var() {},
    void() {},
    while() {},
    with() {},
    yield() {},
}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:612",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "6:16",
              "name": {
                "Ident": {
                  "span": "6:11",
                  "name": "await"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "11:13",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "14:16",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "22:32",
              "name": {
                "Ident": {
                  "span": "22:27",
                  "name": "break"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "27:29",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "30:32",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "38:47",
              "name": {
                "Ident": {
                  "span": "38:42",
                  "name": "case"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "42:44",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "45:47",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "53:63",
              "name": {
                "Ident": {
                  "span": "53:58",
                  "name": "catch"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "58:60",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "61:63",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "69:79",
              "name": {
                "Ident": {
                  "span": "69:74",
                  "name": "class"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "74:76",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "77:79",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "85:95",
              "name": {
                "Ident": {
                  "span": "85:90",
                  "name": "const"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "90:92",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "93:95",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "101:114",
              "name": {
                "Ident": {
                  "span": "101:109",
                  "name": "continue"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "109:111",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "112:114",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "120:133",
              "name": {
                "Ident": {
                  "span": "120:128",
                  "name": "debugger"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "128:130",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "131:133",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "139:151",
              "name": {
                "Ident": {
                  "span": "139:146",
                  "name": "default"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "146:148",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "149:151",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "157:168",
              "name": {
                "Ident": {
                  "span": "157:163",
                  "name": "delete"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "163:165",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "166:168",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "174:181",
              "name": {
                "Ident": {
                  "span": "174:176",
                  "name": "do"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "176:178",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "179:181",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "187:196",
              "name": {
                "Ident": {
                  "span": "187:191",
                  "name": "else"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "191:193",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "194:196",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "202:211",
              "name": {
                "Ident": {
                  "span": "202:206",
                  "name": "enum"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "206:208",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "209:211",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "217:228",
              "name": {
                "Ident": {
                  "span": "217:223",
                  "name": "export"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "223:225",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "226:228",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "234:246",
              "name": {
                "Ident": {
                  "span": "234:241",
                  "name": "extends"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "241:243",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "244:246",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "252:262",
              "name": {
                "Ident": {
                  "span": "252:257",
                  "name": "false"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "257:259",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "260:262",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "268:280",
              "name": {
                "Ident": {
                  "span": "268:275",
                  "name": "finally"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "275:277",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "278:280",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "286:294",
              "name": {
                "Ident": {
                  "span": "286:289",
                  "name": "for"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "289:291",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "292:294",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "300:313",
              "name": {
                "Ident": {
                  "span": "300:308",
                  "name": "function"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "308:310",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "311:313",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "319:326",
              "name": {
                "Ident": {
                  "span": "319:321",
                  "name": "if"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "321:323",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "324:326",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "332:343",
              "name": {
                "Ident": {
                  "span": "332:338",
                  "name": "import"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "338:340",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "341:343",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "349:356",
              "name": {
                "Ident": {
                  "span": "349:351",
                  "name": "in"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "351:353",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "354:356",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "362:377",
              "name": {
                "Ident": {
                  "span": "362:372",
                  "name": "instanceof"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "372:374",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "375:377",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "383:391",
              "name": {
                "Ident": {
                  "span": "383:386",
                  "name": "new"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "386:388",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "389:391",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "397:406",
              "name": {
                "Ident": {
                  "span": "397:401",
                  "name": "null"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "401:403",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "404:406",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "412:423",
              "name": {
                "Ident": {
                  "span": "412:418",
                  "name": "return"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "418:420",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "421:423",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "429:439",
              "name": {
                "Ident": {
                  "span": "429:434",
                  "name": "super"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "434:436",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "437:439",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "445:456",
              "name": {
                "Ident": {
                  "span": "445:451",
                  "name": "switch"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "451:453",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "454:456",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "462:471",
              "name": {
                "Ident": {
                  "span": "462:466",
                  "name": "this"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "466:468",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "469:471",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "477:487",
              "name": {
                "Ident": {
                  "span": "477:482",
                  "name": "throw"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "482:484",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "485:487",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "493:502",
              "name": {
                "Ident": {
                  "span": "493:497",
                  "name": "true"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "497:499",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "500:502",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "508:516",
              "name": {
                "Ident": {
                  "span": "508:511",
                  "name": "try"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "511:513",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "514:516",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "522:533",
              "name": {
                "Ident": {
                  "span": "522:528",
                  "name": "typeof"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "528:530",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "531:533",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "539:547",
              "name": {
                "Ident": {
                  "span": "539:542",
                  "name": "var"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "542:544",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "545:547",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "553:562",
              "name": {
                "Ident": {
                  "span": "553:557",
                  "name": "void"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "557:559",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "560:562",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "568:578",
              "name": {
                "Ident": {
                  "span": "568:573",
                  "name": "while"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "573:575",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "576:578",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "584:593",
              "name": {
                "Ident": {
                  "span": "584:588",
                  "name": "with"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "588:590",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "591:593",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "599:609",
              "name": {
                "Ident": {
                  "span": "599:604",
                  "name": "yield"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "604:606",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "607:609",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          }
        ]
      }
    }
  }
}
```
