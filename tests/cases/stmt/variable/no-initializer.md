### Input
```js
var foo;
```

```js min
var foo
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:8",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:7",
        "pattern": {
          "Ident": {
            "span": "4:7",
            "name": "foo"
          }
        },
        "initializer": null
      }
    ]
  }
}
```
