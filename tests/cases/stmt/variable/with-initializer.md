### Input
```js
var foo = a;
```

### Output: minified
```js min
var foo=a
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:12",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:11",
        "pattern": {
          "Ident": {
            "span": "4:7",
            "name": "foo"
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "10:11",
            "name": "a"
          }
        }
      }
    ]
  }
}
```
