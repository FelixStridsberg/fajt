### Source
```js parse:stmt check-format:no
var { a, } = b;
```

### Output: minified
```js
var{a}=b;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:15",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:14",
        "pattern": {
          "Object": {
            "span": "4:10",
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
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "13:14",
            "name": "b"
          }
        }
      }
    ]
  }
}
```