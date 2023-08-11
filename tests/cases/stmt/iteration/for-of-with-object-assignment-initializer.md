### Source
```js parse:stmt
for ({ a = 1 } of b) ;
```

### Output: minified
```js
for({a=1}of b);
```

### Output: ast
```json
{
  "ForOf": {
    "span": "0:22",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Object": {
            "span": "5:14",
            "props": [
              {
                "Single": {
                  "span": "7:12",
                  "ident": {
                    "span": "7:8",
                    "name": "a"
                  },
                  "initializer": {
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
            ],
            "rest": null
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "18:19",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "21:22"
      }
    },
    "asynchronous": false
  }
}
```
