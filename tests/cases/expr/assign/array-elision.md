### Source
```js parse:expr check-format:no
[ , ] = a
```

### Output: minified
```js
[]=a
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:9",
    "operator": "Assign",
    "left": {
      "Literal": {
        "span": "0:5",
        "literal": {
          "Array": {
            "elements": [
              "Elision"
            ]
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    }
  }
}
```
