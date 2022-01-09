### Source
```js
do {} while(true)
do {} while (true);
do ; while(a) b
```

### Output: ast
```json
{
  "Script": {
    "span": "0:53",
    "body": [
      {
        "DoWhile": {
          "span": "0:17",
          "body": {
            "Block": {
              "span": "3:5",
              "statements": []
            }
          },
          "test": {
            "Literal": {
              "span": "12:16",
              "literal": {
                "Boolean": true
              }
            }
          }
        }
      },
      {
        "DoWhile": {
          "span": "18:37",
          "body": {
            "Block": {
              "span": "21:23",
              "statements": []
            }
          },
          "test": {
            "Literal": {
              "span": "31:35",
              "literal": {
                "Boolean": true
              }
            }
          }
        }
      },
      {
        "DoWhile": {
          "span": "38:51",
          "body": {
            "Empty": {
              "span": "41:42"
            }
          },
          "test": {
            "IdentRef": {
              "span": "49:50",
              "name": "a"
            }
          }
        }
      },
      {
        "Expr": {
          "span": "52:53",
          "expr": {
            "IdentRef": {
              "span": "52:53",
              "name": "b"
            }
          }
        }
      }
    ]
  }
}
```
