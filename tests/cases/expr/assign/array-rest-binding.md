### Source
```js parse:expr
[ ...rest ] = b
```

### Output: minified
```js
[...rest]=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:15",
    "operator": "Assign",
    "left": {
      "ArrayBinding": {
        "span": "0:11",
        "elements": [],
        "rest": {
          "span": "5:9",
          "name": "rest"
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "14:15",
        "name": "b"
      }
    }
  }
}
```
