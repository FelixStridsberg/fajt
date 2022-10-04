### Source
```js parse:stmt
class cls {
    *method1() {
        yield;
    }
}
```

### Output: minified
```js
class cls{*method1(){yield}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:51",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:49",
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
            "span": "27:49",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "37:43",
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
          "asynchronous": false,
          "is_static": false
        }
      }
    ]
  }
}
```
