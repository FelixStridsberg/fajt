### Source
```js parse:stmt
var { a = b } = c;
```

### Output: minified
```js
var{a=b}=c;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:18",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:17",
        "pattern": {
          "Object": {
            "span": "4:13",
            "props": [
              {
                "Single": {
                  "span": "6:11",
                  "ident": {
                    "span": "6:7",
                    "name": "a"
                  },
                  "initializer": {
                    "IdentRef": {
                      "span": "10:11",
                      "name": "b"
                    }
                  }
                }
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "16:17",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
