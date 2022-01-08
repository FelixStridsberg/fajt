### Input
```js
a?.()?.()?.()
```

### Output: ast
```json
{
  "OptionalCall": {
    "span": "0:13",
    "callee": {
      "OptionalCall": {
        "span": "0:9",
        "callee": {
          "OptionalCall": {
            "span": "0:5",
            "callee": {
              "IdentRef": {
                "span": "0:1",
                "name": "a"
              }
            },
            "arguments_span": "3:5",
            "arguments": [],
            "optional": true
          }
        },
        "arguments_span": "7:9",
        "arguments": [],
        "optional": true
      }
    },
    "arguments_span": "11:13",
    "arguments": [],
    "optional": true
  }
}
```
