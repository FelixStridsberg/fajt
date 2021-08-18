```js
class cls {
  *method1() {
    yield
  }
}
```

```json
{
  "Class": {
    "span": "0:42",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "14:40",
          "name": {
            "Ident": {
              "span": "15:22",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:24",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "25:40",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "31:36",
                  "expr": {
                    "Yield": {
                      "span": "31:36",
                      "argument": null,
                      "delegate": false
                    }
                  }
                }
              }
            ]
          },
          "generator": true,
          "asynchronous": false
        }
      }
    ]
  }
}
```
