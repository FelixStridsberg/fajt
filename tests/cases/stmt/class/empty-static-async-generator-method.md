### Source
```js parse:stmt
class cls {
    static async *method1() {
        yield;
    }
}
```

### Output: minified
```js
class cls{static async*method1(){yield}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:64",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:62",
          "name": {
            "Ident": {
              "span": "30:37",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "37:39",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "40:62",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "50:56",
                  "expr": {
                    "Yield": {
                      "span": "50:55",
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
