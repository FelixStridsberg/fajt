```js
var [ { a } ] = b;
```

```js min
var[{a}]=b
```

```json
{
  "Variable": {
    "span": "0:18",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:17",
        "pattern": {
          "Array": {
            "span": "4:13",
            "elements": [
              {
                "span": "6:11",
                "pattern": {
                  "Object": {
                    "span": "6:11",
                    "props": [
                      {
                        "Single": [
                          {
                            "span": "8:9",
                            "name": "a"
                          },
                          null
                        ]
                      }
                    ],
                    "rest": null
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
            "span": "16:17",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
