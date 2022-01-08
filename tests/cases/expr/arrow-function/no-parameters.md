### Input
```js parse:expr
() => {}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:8",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:2",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Body": {
        "span": "6:8",
        "directives": [],
        "statements": []
      }
    }
  }
}
```
