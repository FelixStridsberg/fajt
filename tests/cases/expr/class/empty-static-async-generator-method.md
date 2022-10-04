### Source
```js parse:expr
class {
    static async *method1() {
        yield;
    }
}
```

### Output: minified
```js
class{static async*method1(){yield}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:60",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "19:58",
          "name": {
            "Ident": {
              "span": "26:33",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "33:35",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "36:58",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "46:52",
                  "expr": {
                    "Yield": {
                      "span": "46:51",
                      "argument": null,
                      "delegate": false
                    }
                  }
                }
              }
            ]
          },
          "generator": true,
          "asynchronous": true,
          "is_static": true
        }
      }
    ]
  }
}
```
