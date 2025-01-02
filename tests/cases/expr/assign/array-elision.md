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
      "AssignmentPattern": {
        "Array": {
          "span": "0:5",
          "elements": [
            null
          ],
          "rest": null
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
