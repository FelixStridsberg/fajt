### Source
```js
class Test extends Test2 {
    "constructor"() {
        super();
    }
}
```

### Output: minified
```js
class Test extends Test2{"constructor"(){super()}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:73",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:73",
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
                "span": "31:71",
                "name": {
                  "String": {
                    "value": "constructor",
                    "delimiter": "\""
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "44:46",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "47:71",
                  "directives": [],
                  "statements": [
                    {
                      "Expr": {
                        "span": "57:65",
                        "expr": {
                          "Call": {
                            "span": "57:64",
                            "callee": "Super",
                            "arguments_span": "62:64",
                            "arguments": []
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
