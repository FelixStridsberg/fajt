### Input
```js parse:expr
class {
  *method1() {
    yield
  }
}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:38",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "10:36",
          "name": {
            "Ident": {
              "span": "11:18",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:20",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "21:36",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "27:32",
                  "expr": {
                    "Yield": {
                      "span": "27:32",
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
