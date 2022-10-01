### Source
```js parse:stmt
function fn() {
    "object" == typeof exports;
}
```

### Output: minified
```js
function fn(){"object"==typeof exports}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:49",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:13",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "14:49",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "20:47",
            "expr": {
              "Binary": {
                "span": "20:46",
                "operator": "Equal",
                "left": {
                  "Literal": {
                    "span": "20:28",
                    "literal": {
                      "String": {
                        "value": "object",
                        "delimiter": "\""
                      }
                    }
                  }
                },
                "right": {
                  "Unary": {
                    "span": "32:46",
                    "operator": "Typeof",
                    "argument": {
                      "IdentRef": {
                        "span": "39:46",
                        "name": "exports"
                      }
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
}
```
