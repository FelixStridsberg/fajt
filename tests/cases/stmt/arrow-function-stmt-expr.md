### Source
```js parse:stmt
({a=b}) => 1;
```

### Output: ast
```json
{
  "Expr": {
    "span": "0:13",
    "expr": {
      "ArrowFunction": {
        "span": "0:12",
        "asynchronous": false,
        "binding_parameter": false,
        "parameters": {
          "span": "0:7",
          "bindings": [
            {
              "span": "1:6",
              "pattern": {
                "Object": {
                  "span": "1:6",
                  "props": [
                    {
                      "Single": {
                        "span": "2:5",
                        "ident": {
                          "span": "2:3",
                          "name": "a"
                        },
                        "initializer": {
                          "IdentRef": {
                            "span": "4:5",
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
              "span": "11:12",
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
  }
}
```
