```js
var { a: b } = c;
```

```js min
var{a:b}=c
```

```json
{
  "Variable": {
    "span": "0:17",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:16",
        "pattern": {
          "Object": {
            "span": "4:12",
            "props": [
              {
                "KeyValue": [
                  {
                    "Ident": {
                      "span": "6:7",
                      "name": "a"
                    }
                  },
                  {
                    "span": "9:10",
                    "pattern": {
                      "Ident": {
                        "span": "9:10",
                        "name": "b"
                      }
                    },
                    "initializer": null
                  }
                ]
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "15:16",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
