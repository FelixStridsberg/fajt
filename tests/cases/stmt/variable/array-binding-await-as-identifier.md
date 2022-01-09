### Source
```js parse:stmt
var [ await, ...await ] = c;
```

### Output: minified
```js
var[await,...await]=c
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
                    "name": "await"
                  }
                },
                "initializer": null
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
