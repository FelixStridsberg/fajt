### Source
```js parse:expr
{ a: b } = c
```

### Output: minified
```js
{a:b}=c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:12",
    "operator": "Assign",
    "left": {
      "ObjectBinding": {
        "span": "0:8",
        "props": [
          {
            "Named": {
              "span": "2:6",
              "property": {
                "Ident": {
                  "span": "2:3",
                  "name": "a"
                }
              },
              "binding": {
                "span": "5:6",
                "pattern": {
                  "Ident": {
                    "span": "5:6",
                    "name": "b"
                  }
                },
                "initializer": null
              }
            }
          }
        ],
        "rest": null
      }
    },
    "right": {
      "IdentRef": {
        "span": "11:12",
        "name": "c"
      }
    }
  }
}
```
