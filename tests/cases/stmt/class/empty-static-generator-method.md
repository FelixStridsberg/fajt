### Source
```js parse:stmt
class cls {
    static *method1() {
        yield;
    }
}
```

### Output: minified
```js
class cls{static*method1(){yield}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:58",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:56",
          "name": {
            "Ident": {
              "span": "24:31",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "31:33",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "34:56",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "44:50",
                  "expr": {
                    "Yield": {
                      "span": "44:49",
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
          "is_static": true
        }
      }
    ]
  }
}
```
