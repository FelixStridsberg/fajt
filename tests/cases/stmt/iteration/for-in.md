### Input
```js
for (a in b) ;
```

### Output: minified
```js min
for(a in b);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:14",
    "left": {
      "Expr": {
        "IdentRef": {
          "span": "5:6",
          "name": "a"
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "10:11",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "13:14"
      }
    }
  }
}
```
