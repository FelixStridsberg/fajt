### Source
```js parse:expr
{ [a]: b } = c
```

### Output: minified
```js
{[a]:b}=c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:14",
    "operator": "Assign",
    "left": {
      "ObjectBinding": {
        "span": "0:10",
        "props": [
          {
            "Named": {
              "span": "2:8",
              "property": {
                "Computed": {
                  "IdentRef": {
                    "span": "3:4",
                    "name": "a"
                  }
                }
              },
              "binding": {
                "span": "7:8",
                "pattern": {
                  "Ident": {
                    "span": "7:8",
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
        "span": "13:14",
        "name": "c"
      }
    }
  }
}
```
