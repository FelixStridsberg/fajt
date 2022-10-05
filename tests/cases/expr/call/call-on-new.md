### Source
```js parse:expr
new Date().getTime()
```

### Output: minified
```js
new Date().getTime()
```

### Output: ast
```json
{
  "Call": {
    "span": "0:20",
    "callee": {
      "Expr": {
        "Member": {
          "span": "0:18",
          "object": {
            "Expr": {
              "New": {
                "span": "0:10",
                "callee": {
                  "IdentRef": {
                    "span": "4:8",
                    "name": "Date"
                  }
                },
                "arguments_span": "8:10",
                "arguments": []
              }
            }
          },
          "property": {
            "Ident": {
              "span": "11:18",
              "name": "getTime"
            }
          }
        }
      }
    },
    "arguments_span": "18:20",
    "arguments": []
  }
}
```
