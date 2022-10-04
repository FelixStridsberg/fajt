### Source
```js parse:expr
class {
    static *method1() {
        yield;
    }
}
```

### Output: minified
```js
class{static*method1(){yield}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:54",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "19:52",
          "name": {
            "Ident": {
              "span": "20:27",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "27:29",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "30:52",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "40:46",
                  "expr": {
                    "Yield": {
                      "span": "40:45",
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
