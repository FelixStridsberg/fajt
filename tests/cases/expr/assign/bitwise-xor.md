### Input
```js
a ^= b;
```

```js min
a^=b
```

```json
{
  "Assignment": {
    "span": "0:6",
    "operator": "BitwiseXOr",
    "left": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "right": {
      "IdentRef": {
        "span": "5:6",
        "name": "b"
      }
    }
  }
}
```
