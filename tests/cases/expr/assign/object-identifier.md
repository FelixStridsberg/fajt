### Source
```js parse:expr
{ a } = c
```

### Output: minified
```js
{a}=c
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
          "Object": {
            "props": [
              {
                "IdentRef": {
                  "span": "2:3",
                  "name": "a"
                }
              }
            ]
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
