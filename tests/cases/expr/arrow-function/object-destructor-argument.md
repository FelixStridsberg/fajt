### Source
```js parse:expr
({a}) => 1
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:10",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:5",
      "bindings": [
        {
          "span": "1:4",
          "pattern": {
            "Object": {
              "span": "1:4",
              "props": [
                {
                  "Single": {
                    "span": "2:3",
                    "ident": {
                      "span": "2:3",
                      "name": "a"
                    },
                    "initializer": null
                  }
                }
              ],
              "rest": null
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "Expr": {
        "Literal": {
          "span": "9:10",
          "literal": {
            "Number": {
              "raw": "1"
            }
          }
        }
      }
    }
  }
}
```
