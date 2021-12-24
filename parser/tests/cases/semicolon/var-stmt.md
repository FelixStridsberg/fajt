```js
var a
var a;
```

```json
{
  "Script": {
    "span": "0:12",
    "body": [
      {
        "Variable": {
          "span": "0:5",
          "kind": "Var",
          "declarations": [
            {
              "span": "4:5",
              "pattern": {
                "Ident": {
                  "span": "4:5",
                  "name": "a"
                }
              },
              "initializer": null
            }
          ]
        }
      },
      {
        "Variable": {
          "span": "6:12",
          "kind": "Var",
          "declarations": [
            {
              "span": "10:11",
              "pattern": {
                "Ident": {
                  "span": "10:11",
                  "name": "a"
                }
              },
              "initializer": null
            }
          ]
        }
      }
    ]
  }
}
```
