### Input
```js parse:stmt
var [ yield, ...yield ] = c;
```

### Output: minified
```js
var[yield,...yield]=c
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:28",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:27",
        "pattern": {
          "Array": {
            "span": "4:23",
            "elements": [
              {
                "span": "6:11",
                "pattern": {
                  "Ident": {
                    "span": "6:11",
                    "name": "yield"
                  }
                },
                "initializer": null
              }
            ],
            "rest": {
              "span": "16:21",
              "name": "yield"
            }
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "26:27",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
