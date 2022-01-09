### Source
```js parse:expr
async () => {}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:14",
    "asynchronous": true,
    "binding_parameter": false,
    "parameters": {
      "span": "6:8",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Body": {
        "span": "12:14",
        "directives": [],
        "statements": []
      }
    }
  }
}
```
