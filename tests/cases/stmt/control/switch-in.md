### Source
```js parse:stmt
switch ("a" in b) {
    case "c" in d:
}
```

### Output: minified
```js
switch("a"in b){case "c"in d:}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:40",
    "discriminant": {
      "Binary": {
        "span": "8:16",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "8:11",
            "literal": {
              "String": {
                "value": "a",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "15:16",
            "name": "b"
          }
        }
      }
    },
    "cases": [
      {
        "span": "24:38",
        "test": {
          "Binary": {
            "span": "29:37",
            "operator": "In",
            "left": {
              "Literal": {
                "span": "29:32",
                "literal": {
                  "String": {
                    "value": "c",
                    "delimiter": "\""
                  }
                }
              }
            },
            "right": {
              "IdentRef": {
                "span": "36:37",
                "name": "d"
              }
            }
          }
        },
        "consequent": []
      }
    ]
  }
}
```
