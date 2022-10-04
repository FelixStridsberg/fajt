### Source
```js
class Test extends Test2 {
    constructor() {
        super.a;
    }
}
```

### Output: minified
```js
class Test extends Test2{constructor(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:71",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:71",
          "identifier": {
            "span": "6:10",
            "name": "Test"
          },
          "super_class": {
            "IdentRef": {
              "span": "19:24",
              "name": "Test2"
            }
          },
          "body": [
            {
              "Method": {
                "span": "31:69",
                "name": {
                  "Ident": {
                    "span": "31:42",
                    "name": "constructor"
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "42:44",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "45:69",
                  "directives": [],
                  "statements": [
                    {
                      "Expr": {
                        "span": "55:63",
                        "expr": {
                          "Member": {
                            "span": "55:62",
                            "object": {
                              "Super": {
                                "span": "55:60"
                              }
                            },
                            "property": {
                              "Ident": {
                                "span": "61:62",
                                "name": "a"
                              }
                            }
                          }
                        }
                      }
                    }
                  ]
                },
                "generator": false,
                "asynchronous": false,
                "is_static": false
              }
            }
          ]
        }
      }
    ]
  }
}
```
