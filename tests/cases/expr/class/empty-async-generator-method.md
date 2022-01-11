### Source
```js parse:expr
class {
    async *method1() {
        yield;
    }
}
```

### Output: minified
```js
class{async*method1(){yield}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:53",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:51",
          "name": {
            "Ident": {
              "span": "19:26",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "26:28",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "29:51",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "39:45",
                  "expr": {
                    "Yield": {
                      "span": "39:44",
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
