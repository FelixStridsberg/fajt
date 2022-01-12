### Source
```js parse:stmt
var { a: b } = c;
```

### Output: minified
```js
var{a:b}=c;
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
                "Named": {
                  "span": "6:10",
                  "property": {
                    "Ident": {
                      "span": "6:7",
                      "name": "a"
                    }
                  },
                  "binding": {
                    "span": "9:10",
                    "pattern": {
                      "Ident": {
                        "span": "9:10",
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
