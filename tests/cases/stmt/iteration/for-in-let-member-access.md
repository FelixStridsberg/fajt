### Source
```js parse:stmt
for (let.let in b) ;
```

### Output: minified
```js
for(let.let in b);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:20",
    "left": {
      "Expr": {
        "Member": {
          "span": "5:12",
          "object": {
            "Expr": {
              "IdentRef": {
                "span": "5:8",
                "name": "let"
              }
            }
          },
          "property": {
            "Ident": {
              "span": "9:12",
              "name": "let"
            }
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "16:17",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "19:20"
      }
    }
  }
}
```
