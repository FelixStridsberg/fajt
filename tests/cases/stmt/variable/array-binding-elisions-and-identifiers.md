```js
var [ , a,,b ] = c;
```

```json
{
  "Variable": {
    "span": "0:19",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:18",
        "pattern": {
          "Array": {
            "span": "4:14",
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
                "span": "11:12",
                "pattern": {
                  "Ident": {
                    "span": "11:12",
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
            "span": "17:18",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
