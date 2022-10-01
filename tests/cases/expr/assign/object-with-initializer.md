### Source
```js parse:expr
{ a = b } = c
```

### Output: minified
```js
{a=b}=c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:13",
    "operator": "Assign",
    "left": {
      "ObjectBinding": {
        "span": "0:9",
        "props": [
          {
            "Single": {
              "span": "2:7",
              "ident": {
                "span": "2:3",
                "name": "a"
              },
              "initializer": {
                "IdentRef": {
                  "span": "6:7",
                  "name": "b"
                }
              }
            }
          }
        ],
        "rest": null
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
