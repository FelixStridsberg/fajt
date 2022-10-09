### Source
```js parse:stmt
var { [a]: b } = c;
```

### Output: minified
```js
var{[a]:b}=c;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:19",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:18",
        "pattern": {
          "Object": {
            "span": "4:14",
            "props": [
              {
                "Named": {
                  "span": "6:12",
                  "property": {
                    "Computed": {
                      "IdentRef": {
                        "span": "7:8",
                        "name": "a"
                      }
                    }
                  },
                  "binding": {
                    "span": "11:12",
                    "pattern": {
                      "Ident": {
                        "span": "11:12",
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
            "span": "17:18",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
