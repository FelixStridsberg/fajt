### Input
```js
class {
  async *method1() {
    yield
  }
}
```

```json
{
  "Class": {
    "span": "0:44",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "10:42",
          "name": {
            "Ident": {
              "span": "17:24",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "24:26",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "27:42",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "33:38",
                  "expr": {
                    "Yield": {
                      "span": "33:38",
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
