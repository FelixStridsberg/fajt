### Source
```js
class Test extends Test2 {
    constructor(a = super.a) {}
}
```

### Output: minified
```js
class Test extends Test2{constructor(a=super.a){}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:60",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:60",
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
                "span": "31:58",
                "name": {
                  "Ident": {
                    "span": "31:42",
                    "name": "constructor"
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "42:55",
                  "bindings": [
                    {
                      "span": "43:54",
                      "pattern": {
                        "Ident": {
                          "span": "43:44",
                          "name": "a"
                        }
                      },
                      "initializer": {
                        "Member": {
                          "span": "47:54",
                          "object": {
                            "Super": {
                              "span": "47:52"
                            }
                          },
                          "property": {
                            "Ident": {
                              "span": "53:54",
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
                  "span": "56:58",
                  "directives": [],
                  "statements": []
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
