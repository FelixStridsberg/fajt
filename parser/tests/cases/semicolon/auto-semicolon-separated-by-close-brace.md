### Input
```js
{ a } b
```

```json
{
  "Script": {
    "span": "0:7",
    "body": [
      {
        "Block": {
          "span": "0:5",
          "statements": [
            {
              "Expr": {
                "span": "2:3",
                "expr": {
                  "IdentRef": {
                    "span": "2:3",
                    "name": "a"
                  }
                }
              }
            }
          ]
        }
      },
      {
        "Expr": {
          "span": "6:7",
          "expr": {
            "IdentRef": {
              "span": "6:7",
              "name": "b"
            }
          }
        }
      }
    ]
  }
}
```
