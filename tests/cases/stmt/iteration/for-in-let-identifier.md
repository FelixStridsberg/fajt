### Source
```js parse:stmt
for (let in b) ;
```

### Output: minified
```js
for(let in b);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:16",
    "left": {
      "Expr": {
        "IdentRef": {
          "span": "5:8",
          "name": "let"
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
