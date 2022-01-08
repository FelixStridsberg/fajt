### Input
```js parse:stmt
var { a: { b } } = c;
```

### Output: minified
```js
var{a:{b}}=c
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:21",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:20",
        "pattern": {
          "Object": {
            "span": "4:16",
            "props": [
              {
                "KeyValue": [
                  {
                    "Ident": {
                      "span": "6:7",
                      "name": "a"
                    }
                  },
                  {
                    "span": "9:14",
                    "pattern": {
                      "Object": {
                        "span": "9:14",
                        "props": [
                          {
                            "Single": [
                              {
                                "span": "11:12",
                                "name": "b"
                              },
                              null
                            ]
                          }
                        ],
                        "rest": null
                      }
                    },
                    "initializer": null
                  }
                ]
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "19:20",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
