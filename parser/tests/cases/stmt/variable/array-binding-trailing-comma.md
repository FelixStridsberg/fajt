### Input
```js
var [ a, ] = b;
```

```json
{
  "Variable": {
    "span": "0:15",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:14",
        "pattern": {
          "Array": {
            "span": "4:10",
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
            "span": "13:14",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
