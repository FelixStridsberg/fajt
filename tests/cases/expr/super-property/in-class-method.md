### Source
```js
class Test extends Test2 {
    method() {
        super.a;
    }
}
```

### Output: minified
```js
class Test extends Test2{method(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:66",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:66",
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
                "span": "31:64",
                "name": {
                  "Ident": {
                    "span": "31:37",
                    "name": "method"
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "37:39",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "40:64",
                  "directives": [],
                  "statements": [
                    {
                      "Expr": {
                        "span": "50:58",
                        "expr": {
                          "Member": {
                            "span": "50:57",
                            "object": {
                              "Super": {
                                "span": "50:55"
                              }
                            },
                            "property": {
                              "Ident": {
                                "span": "56:57",
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
                "asynchronous": false
              }
            }
          ]
        }
      }
    ]
  }
}
```
