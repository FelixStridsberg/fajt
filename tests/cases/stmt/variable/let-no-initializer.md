### Input
```js parse:stmt
let foo;
```

### Output: minified
```js
let foo
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:8",
    "kind": "Let",
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
