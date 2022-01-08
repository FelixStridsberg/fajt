### Input
```js
var a = b,
    c;
```

### Output: minified
```js
var a=b,c
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:17",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:9",
        "pattern": {
          "Ident": {
            "span": "4:5",
            "name": "a"
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "8:9",
            "name": "b"
          }
        }
      },
      {
        "span": "15:16",
        "pattern": {
          "Ident": {
            "span": "15:16",
            "name": "c"
          }
        },
        "initializer": null
      }
    ]
  }
}
```
