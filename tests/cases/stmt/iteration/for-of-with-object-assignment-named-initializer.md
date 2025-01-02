### Source
```js parse:stmt
for ({ a: b = 1 } of c) ;
```

### Output: minified
```js
for({a=1}of b);
```

### Output: ast
```json
{
  "ForOf": {
    "span": "0:25",
    "left": {
      "AssignmentPattern": {
        "Object": {
          "span": "5:17",
          "props": [
            {
              "Named": {
                "span": "7:15",
                "name": {
                  "Ident": {
                    "span": "7:8",
                    "name": "a"
                  }
                },
                "value": {
                  "Expr": {
                    "IdentRef": {
                      "span": "10:11",
                      "name": "b"
                    }
                  }
                },
                "initializer": {
                  "Literal": {
                    "span": "14:15",
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
    },
    "right": {
      "IdentRef": {
        "span": "21:22",
        "name": "c"
      }
    },
    "body": {
      "Empty": {
        "span": "24:25"
      }
    },
    "asynchronous": false
  }
}
```
