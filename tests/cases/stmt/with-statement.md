### Input
```js
with({}) {}
```

```js min
with({}){}
```

### Output: ast
```json
{
  "With": {
    "span": "0:11",
    "object": {
      "Literal": {
        "span": "5:7",
        "literal": {
          "Object": {
            "props": []
          }
        }
      }
    },
    "body": {
      "Block": {
        "span": "9:11",
        "statements": []
      }
    }
  }
}
```
