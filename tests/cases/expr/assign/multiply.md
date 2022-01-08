### Input
```js
a *= b;
```

```js min
a*=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:6",
    "operator": "Multiply",
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
