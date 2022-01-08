### Input
```js
do ; while(a) b
```

```json
{
  "Script": {
    "span": "0:15",
    "body": [
      {
        "DoWhile": {
          "span": "0:13",
          "body": {
            "Empty": {
              "span": "3:4"
            }
          },
          "test": {
            "IdentRef": {
              "span": "11:12",
              "name": "a"
            }
          }
        }
      },
      {
        "Expr": {
          "span": "14:15",
          "expr": {
            "IdentRef": {
              "span": "14:15",
              "name": "b"
            }
          }
        }
      }
    ]
  }
}
```
