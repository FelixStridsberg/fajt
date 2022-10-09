### Source
```js
var a = ("b" in c);
```

### Output: ast
```json
{
  "Script": {
    "span": "0:19",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:19",
          "kind": "Var",
          "declarations": [
            {
              "span": "4:18",
              "pattern": {
                "Ident": {
                  "span": "4:5",
                  "name": "a"
                }
              },
              "initializer": {
                "Parenthesized": {
                  "span": "8:18",
                  "expression": {
                    "Binary": {
                      "span": "9:17",
                      "operator": "In",
                      "left": {
                        "Literal": {
                          "span": "9:12",
                          "literal": {
                            "String": {
                              "value": "b",
                              "delimiter": "\""
                            }
                          }
                        }
                      },
                      "right": {
                        "IdentRef": {
                          "span": "16:17",
                          "name": "c"
                        }
                      }
                    }
                  }
                }
              }
            }
          ]
        }
      }
    ]
  }
}
```
