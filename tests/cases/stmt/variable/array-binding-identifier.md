### Input
```js
var [ a ] = b;
```

### Output: minified
```js min
var[a]=b
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:14",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:13",
        "pattern": {
          "Array": {
            "span": "4:9",
            "elements": [
              {
                "span": "6:7",
                "pattern": {
                  "Ident": {
                    "span": "6:7",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "12:13",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
