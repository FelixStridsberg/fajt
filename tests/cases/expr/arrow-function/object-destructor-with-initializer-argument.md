### Source
```js parse:expr
({a = b}) => 1
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:14",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:9",
      "bindings": [
        {
          "span": "1:8",
          "pattern": {
            "Object": {
              "span": "1:8",
              "props": [
                {
                  "Single": {
                    "span": "2:7",
                    "ident": {
                      "span": "2:3",
                      "name": "a"
                    },
                    "initializer": {
                      "IdentRef": {
                        "span": "6:7",
                        "name": "b"
                      }
                    }
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
          "span": "13:14",
          "literal": {
            "Number": {
              "Integer": [
                1,
                "Decimal"
              ]
            }
          }
        }
      }
    }
  }
}
```
