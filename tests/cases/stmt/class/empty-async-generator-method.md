```js
class cls {
    async *method1() {
        yield;
    }
}
```

```js min
class cls{async*method1(){yield}}
```

```json
{
  "Class": {
    "span": "0:57",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:55",
          "name": {
            "Ident": {
              "span": "23:30",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "30:32",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "33:55",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "43:49",
                  "expr": {
                    "Yield": {
                      "span": "43:48",
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
