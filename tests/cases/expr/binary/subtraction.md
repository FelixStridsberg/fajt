### Input
```js
a - b;
```

### Output: minified
```js min
a-b
```

### Output: ast
```json
{
  "Binary": {
    "span": "0:5",
    "operator": "Minus",
    "left": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "right": {
      "IdentRef": {
        "span": "4:5",
        "name": "b"
      }
    }
  }
}
```
