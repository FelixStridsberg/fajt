```js
var a = b, c;
```

```json
{
  "Variable": {
    "span": "0:13",
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
        "span": "11:12",
        "pattern": {
          "Ident": {
            "span": "11:12",
            "name": "c"
          }
        },
        "initializer": null
      }
    ]
  }
}
```
