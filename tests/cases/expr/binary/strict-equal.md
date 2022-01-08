### Input
```js
a === b === c;
```

### Output: minified
```js min
a===b===c
```

### Output: ast
```json
{
  "Binary": {
    "span": "0:13",
    "operator": "StrictEqual",
    "left": {
      "Binary": {
        "span": "0:7",
        "operator": "StrictEqual",
        "left": {
          "IdentRef": {
            "span": "0:1",
            "name": "a"
          }
        },
        "right": {
          "IdentRef": {
            "span": "6:7",
            "name": "b"
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "12:13",
        "name": "c"
      }
    }
  }
}
```
