### Source
```js
class Test extends Test2 {
    "constructor"(a = super()) {}
}
```

### Output: minified
```js
class Test extends Test2{"constructor"(a=super()){}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:62",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:62",
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
                "span": "31:60",
                "name": {
                  "String": {
                    "value": "constructor",
                    "delimiter": "\""
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "44:57",
                  "bindings": [
                    {
                      "span": "45:56",
                      "pattern": {
                        "Ident": {
                          "span": "45:46",
                          "name": "a"
                        }
                      },
                      "initializer": {
                        "Call": {
                          "span": "49:56",
                          "callee": "Super",
                          "arguments_span": "54:56",
                          "arguments": []
                        }
                      }
                    }
                  ],
                  "rest": null
                },
                "body": {
                  "span": "58:60",
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
