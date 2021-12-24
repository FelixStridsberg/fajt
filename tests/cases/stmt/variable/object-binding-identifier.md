```js
var { a } = b;
```

```js min
var{a}=b
```

```json
{
  "Variable": {
    "span": "0:14",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:13",
        "pattern": {
          "Object": {
            "span": "4:9",
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
            "span": "12:13",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
