```js
var { a, } = b;
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
          "Object": {
            "span": "4:10",
            "props": [
              {
                "Single": [
                  {
                    "span": "6:7",
                    "name": "a"
                  },
                  null
                ]
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