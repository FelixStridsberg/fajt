```js
a?.()?.()()
```

```json
{
  "OptionalCall": {
    "span": "0:11",
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
    "arguments_span": "9:11",
    "arguments": [],
    "optional": false
  }
}
```
