```js
var { ...rest } = c;
```

```json
{
  "Variable": {
    "span": "0:20",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:19",
        "pattern": {
          "Object": {
            "span": "4:15",
            "props": [],
            "rest": {
              "span": "9:13",
              "name": "rest"
            }
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "18:19",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
