### Source
```js check-format:no
while (true) {if (1) {break} break}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:35",
    "directives": [],
    "body": [
      {
        "While": {
          "span": "0:35",
          "test": {
            "Literal": {
              "span": "7:11",
              "literal": {
                "Boolean": true
              }
            }
          },
          "body": {
            "Block": {
              "span": "13:35",
              "statements": [
                {
                  "If": {
                    "span": "14:28",
                    "condition": {
                      "Literal": {
                        "span": "18:19",
                        "literal": {
                          "Number": {
                            "raw": "1"
                          }
                        }
                      }
                    },
                    "consequent": {
                      "Block": {
                        "span": "21:28",
                        "statements": [
                          {
                            "Break": {
                              "span": "22:27",
                              "label": null
                            }
                          }
                        ]
                      }
                    },
                    "alternate": null
                  }
                },
                {
                  "Break": {
                    "span": "29:34",
                    "label": null
                  }
                }
              ]
            }
          }
        }
      }
    ]
  }
}
```
