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
      "Object": {
        "span": "0:5",
        "props": [
          {
            "Single": {
              "span": "2:3",
              "ident": {
                "span": "2:3",
                "name": "a"
              },
              "initializer": null
            }
          }
        ],
        "rest": null
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
