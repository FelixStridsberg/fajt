### Input
```js
a += b;
```

```json
{
  "Assignment": {
    "span": "0:6",
    "operator": "Add",
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
