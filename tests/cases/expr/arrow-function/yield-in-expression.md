### Source
```js parse:expr
() => yield
```

### Output: minified
```js
()=>yield
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:11",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:2",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Expr": {
        "IdentRef": {
          "span": "6:11",
          "name": "yield"
        }
      }
    }
  }
}
```
