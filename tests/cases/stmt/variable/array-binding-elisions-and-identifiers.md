```js
var [ , a, , b ] = c;
```

```json
{
  "Variable": {
    "span": "0:21",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:20",
        "pattern": {
          "Array": {
            "span": "4:16",
            "elements": [
              null,
              {
                "span": "8:9",
                "pattern": {
                  "Ident": {
                    "span": "8:9",
                    "name": "a"
                  }
                },
                "initializer": null
              },
              null,
              {
                "span": "13:14",
                "pattern": {
                  "Ident": {
                    "span": "13:14",
                    "name": "b"
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
            "span": "19:20",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
