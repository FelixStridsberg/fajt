### Source
```js
switch (a) {
    case 1:
        let b = 2;
}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:45",
    "directives": [],
    "body": [
      {
        "Switch": {
          "span": "0:45",
          "discriminant": {
            "IdentRef": {
              "span": "8:9",
              "name": "a"
            }
          },
          "cases": [
            {
              "span": "17:43",
              "test": {
                "Literal": {
                  "span": "22:23",
                  "literal": {
                    "Number": {
                      "Integer": [
                        1,
                        "Decimal"
                      ]
                    }
                  }
                }
              },
              "consequent": [
                {
                  "Variable": {
                    "span": "33:43",
                    "kind": "Let",
                    "declarations": [
                      {
                        "span": "37:42",
                        "pattern": {
                          "Ident": {
                            "span": "37:38",
                            "name": "b"
                          }
                        },
                        "initializer": {
                          "Literal": {
                            "span": "41:42",
                            "literal": {
                              "Number": {
                                "Integer": [
                                  2,
                                  "Decimal"
                                ]
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
          ]
        }
      }
    ]
  }
}
```
