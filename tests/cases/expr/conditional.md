### Input
```js
test ? consequent : alternate
```

### Output: ast
```json
{
  "Conditional": {
    "span": "0:29",
    "condition": {
      "IdentRef": {
        "span": "0:4",
        "name": "test"
      }
    },
    "consequent": {
      "IdentRef": {
        "span": "7:17",
        "name": "consequent"
      }
    },
    "alternate": {
      "IdentRef": {
        "span": "20:29",
        "name": "alternate"
      }
    }
  }
}
```
