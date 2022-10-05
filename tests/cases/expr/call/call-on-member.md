### Source
```js parse:expr
obj.call()
```

### Output: minified
```js
obj.call()
```

### Output: ast
```json
{
  "Call": {
    "span": "0:10",
    "callee": {
      "Expr": {
        "Member": {
          "span": "0:8",
          "object": {
            "Expr": {
              "IdentRef": {
                "span": "0:3",
                "name": "obj"
              }
            }
          },
          "property": {
            "Ident": {
              "span": "4:8",
              "name": "call"
            }
          }
        }
      }
    },
    "arguments_span": "8:10",
    "arguments": []
  }
}
```
