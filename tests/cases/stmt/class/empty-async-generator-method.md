```js
class cls {
  async *method1() {
    yield
  }
}
```

```json
{
  "Class": {
    "span": "0:48",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "14:46",
          "name": {
            "Ident": {
              "span": "21:28",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "28:30",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "31:46",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "37:42",
                  "expr": {
                    "Yield": {
                      "span": "37:42",
                      "argument": null,
                      "delegate": false
                    }
                  }
                }
              }
            ]
          },
          "generator": true,
          "asynchronous": true
        }
      }
    ]
  }
}
```
