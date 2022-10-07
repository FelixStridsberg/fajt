### Source
```js parse:stmt
while ("a" in b) c;
```

### Output: minified
```js
while("a"in b)c;
```

### Output: ast
```json
{
  "While": {
    "span": "0:19",
    "test": {
      "Binary": {
        "span": "7:15",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "7:10",
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
            "span": "14:15",
            "name": "b"
          }
        }
      }
    },
    "body": {
      "Expr": {
        "span": "17:19",
        "expr": {
          "IdentRef": {
            "span": "17:18",
            "name": "c"
          }
        }
      }
    }
  }
}
```
