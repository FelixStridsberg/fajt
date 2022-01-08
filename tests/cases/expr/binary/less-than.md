### Input
```js
a < b < c;
```

### Output: minified
```js min
a<b<c
```

### Output: ast
```json
{
  "Binary": {
    "span": "0:9",
    "operator": "LessThan",
    "left": {
      "Binary": {
        "span": "0:5",
        "operator": "LessThan",
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
    },
    "right": {
      "IdentRef": {
        "span": "8:9",
        "name": "c"
      }
    }
  }
}
```
