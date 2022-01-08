### Input
```js
var { yield, ...yield } = c;
```

```js min
var{yield,...yield}=c
```

```json
{
  "Variable": {
    "span": "0:28",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:27",
        "pattern": {
          "Object": {
            "span": "4:23",
            "props": [
              {
                "Single": [
                  {
                    "span": "6:11",
                    "name": "yield"
                  },
                  null
                ]
              }
            ],
            "rest": {
              "span": "16:21",
              "name": "yield"
            }
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "26:27",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
