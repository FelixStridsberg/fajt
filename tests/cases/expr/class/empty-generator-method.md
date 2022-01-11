### Source
```js parse:expr
class {
    *method1() {
        yield;
    }
}
```

### Output: minified
```js
class{*method1(){yield}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:47",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:45",
          "name": {
            "Ident": {
              "span": "13:20",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "20:22",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "23:45",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "33:39",
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
          "asynchronous": false
        }
      }
    ]
  }
}
```
