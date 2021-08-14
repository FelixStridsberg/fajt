```js
var [ ...a ] = b;
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
          "Array": {
            "span": "4:12",
            "elements": [],
            "rest": {
              "span": "9:10",
              "name": "a"
            }
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "15:16",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
