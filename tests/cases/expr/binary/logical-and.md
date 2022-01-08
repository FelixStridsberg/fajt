### Input
```js parse:expr
a && b && c;
```

### Output: minified
```js
a&&b&&c
```

### Output: ast
```json
{
  "Logical": {
    "span": "0:11",
    "operator": "And",
    "left": {
      "Logical": {
        "span": "0:6",
        "operator": "And",
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
    },
    "right": {
      "IdentRef": {
        "span": "10:11",
        "name": "c"
      }
    }
  }
}
```
