### Source
```js
class Test extends Test2 {
    method(a = super.a) {}
}
```

### Output: minified
```js
class Test extends Test2{method(a=super.a){}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:55",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:55",
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
                "span": "31:53",
                "name": {
                  "Ident": {
                    "span": "31:37",
                    "name": "method"
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "37:50",
                  "bindings": [
                    {
                      "span": "38:49",
                      "pattern": {
                        "Ident": {
                          "span": "38:39",
                          "name": "a"
                        }
                      },
                      "initializer": {
                        "Member": {
                          "span": "42:49",
                          "object": {
                            "Super": {
                              "span": "42:47"
                            }
                          },
                          "property": {
                            "Ident": {
                              "span": "48:49",
                              "name": "a"
                            }
                          }
                        }
                      }
                    }
                  ],
                  "rest": null
                },
                "body": {
                  "span": "51:53",
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
    ]
  }
}
```
