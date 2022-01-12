### Source
```js parse:stmt
var { await, ...await } = c;
```

### Output: minified
```js
var{await,...await}=c;
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
          "Object": {
            "span": "4:23",
            "props": [
              {
                "Single": {
                  "span": "6:11",
                  "ident": {
                    "span": "6:11",
                    "name": "await"
                  },
                  "initializer": null
                }
              }
            ],
            "rest": {
              "span": "16:21",
              "name": "await"
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
