### Source
```js check-format:no
while (true) {if (1) {continue} continue}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:41",
    "directives": [],
    "body": [
      {
        "While": {
          "span": "0:41",
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
              "span": "13:41",
              "statements": [
                {
                  "If": {
                    "span": "14:31",
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
                        "span": "21:31",
                        "statements": [
                          {
                            "Continue": {
                              "span": "22:30",
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
                  "Continue": {
                    "span": "32:40",
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
