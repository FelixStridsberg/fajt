```js
f1().f2();
```

```js min
f1().f2();
```

```json
{
  "Call": {
    "span": "0:9",
    "callee": {
      "Expr": {
        "Member": {
          "span": "0:7",
          "object": {
            "Expr": {
              "Call": {
                "span": "0:4",
                "callee": {
                  "Expr": {
                    "IdentRef": {
                      "span": "0:2",
                      "name": "f1"
                    }
                  }
                },
                "arguments_span": "2:4",
                "arguments": []
              }
            }
          },
          "property": {
            "Ident": {
              "span": "5:7",
              "name": "f2"
            }
          }
        }
      }
    },
    "arguments_span": "7:9",
    "arguments": []
  }
}
```
