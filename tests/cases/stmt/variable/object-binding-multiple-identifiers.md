### Source
```js parse:stmt
var { a, b } = c;
```

### Output: minified
```js
var{a,b}=c;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:17",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:16",
        "pattern": {
          "Object": {
            "span": "4:12",
            "props": [
              {
                "Single": {
                  "span": "6:7",
                  "ident": {
                    "span": "6:7",
                    "name": "a"
                  },
                  "initializer": null
                }
              },
              {
                "Single": {
                  "span": "9:10",
                  "ident": {
                    "span": "9:10",
                    "name": "b"
                  },
                  "initializer": null
                }
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "15:16",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
