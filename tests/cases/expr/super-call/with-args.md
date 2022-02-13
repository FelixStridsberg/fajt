### Source
```js parse:expr
class Test extends Test2 {
    constructor() {
        super(a, b);
    }
}
```

### Output: minified
```js
class Test extends Test2{constructor(){super(a,b)}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:75",
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
          "span": "31:73",
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
            "span": "45:73",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "55:67",
                  "expr": {
                    "Call": {
                      "span": "55:66",
                      "callee": "Super",
                      "arguments_span": "60:66",
                      "arguments": [
                        {
                          "Expr": {
                            "IdentRef": {
                              "span": "61:62",
                              "name": "a"
                            }
                          }
                        },
                        {
                          "Expr": {
                            "IdentRef": {
                              "span": "64:65",
                              "name": "b"
                            }
                          }
                        }
                      ]
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
```
