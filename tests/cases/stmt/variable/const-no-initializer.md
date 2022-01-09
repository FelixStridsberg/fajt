### Source
```js parse:stmt
const foo;
```

### Output: minified
```js
const foo;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:10",
    "kind": "Const",
    "declarations": [
      {
        "span": "6:9",
        "pattern": {
          "Ident": {
            "span": "6:9",
            "name": "foo"
          }
        },
        "initializer": null
      }
    ]
  }
}
```
