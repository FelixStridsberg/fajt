### Input
```js
for (a of b) ;
```

### Output: minified
```js min
for(a of b);
```

### Output: ast
```json
{
  "ForOf": {
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
    },
    "asynchronous": false
  }
}
```
