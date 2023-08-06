### Source
```js parse:stmt
var { ...[ a ] } = c;
```

### Output: minified
```js
var{...[a]}=c;
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
            "props": [],
            "rest": {
              "Array": {
                "span": "9:14",
                "elements": [
                  {
                    "span": "11:12",
                    "pattern": {
                      "Ident": {
                        "span": "11:12",
                        "name": "a"
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              }
            }
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
