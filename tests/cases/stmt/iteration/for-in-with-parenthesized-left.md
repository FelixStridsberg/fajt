### Source
```js parse:stmt
for ((a) in b) ;
```

### Output: minified
```js
for((a)in b);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:16",
    "left": {
      "Expr": {
        "Parenthesized": {
          "span": "5:8",
          "expression": {
            "IdentRef": {
              "span": "6:7",
              "name": "a"
            }
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "12:13",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "15:16"
      }
    }
  }
}
```
