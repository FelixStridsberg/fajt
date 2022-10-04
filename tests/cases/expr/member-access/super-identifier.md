### Source
TODO bad formatting

```js parse:expr check-format:no
{
    method() {
        super.b
    }
}
```

### Output: minified
```js
{method(){super.b}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:40",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "6:38",
              "name": {
                "Ident": {
                  "span": "6:12",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "12:14",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "15:38",
                "directives": [],
                "statements": [
                  {
                    "Expr": {
                      "span": "25:32",
                      "expr": {
                        "Member": {
                          "span": "25:32",
                          "object": {
                            "Super": {
                              "span": "25:30"
                            }
                          },
                          "property": {
                            "Ident": {
                              "span": "31:32",
                              "name": "b"
                            }
                          }
                        }
                      }
                    }
                  }
                ]
              },
              "generator": false,
              "asynchronous": false,
              "is_static": false
            }
          }
        ]
      }
    }
  }
}
```
