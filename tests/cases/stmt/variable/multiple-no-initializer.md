### Source
```js parse:stmt
var a, b;
```

### Output: minified
```js
var a, b;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:9",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:5",
        "pattern": {
          "Ident": {
            "span": "4:5",
            "name": "a"
          }
        },
        "initializer": null
      },
      {
        "span": "7:8",
        "pattern": {
          "Ident": {
            "span": "7:8",
            "name": "b"
          }
        },
        "initializer": null
      }
    ]
  }
}
```
