### Source
```js parse:stmt
do a;
while ("b" in c)
```

### Output: ast
```json
{
  "DoWhile": {
    "span": "0:22",
    "body": {
      "Expr": {
        "span": "3:5",
        "expr": {
          "IdentRef": {
            "span": "3:4",
            "name": "a"
          }
        }
      }
    },
    "test": {
      "Binary": {
        "span": "13:21",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "13:16",
            "literal": {
              "String": {
                "value": "b",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "20:21",
            "name": "c"
          }
        }
      }
    }
  }
}
```
