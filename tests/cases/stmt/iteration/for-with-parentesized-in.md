### Source
```js parse:stmt
for (var a = ("b" in c);;) ;
```

### Output: minified
```js
for(var a=("b"in c);;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:28",
    "init": {
      "Declaration": {
        "span": "5:23",
        "kind": "Var",
        "declarations": [
          {
            "span": "9:23",
            "pattern": {
              "Ident": {
                "span": "9:10",
                "name": "a"
              }
            },
            "initializer": {
              "Parenthesized": {
                "span": "13:23",
                "expression": {
                  "Binary": {
                    "span": "14:22",
                    "operator": "In",
                    "left": {
                      "Literal": {
                        "span": "14:17",
                        "literal": {
                          "String": {
                            "value": "b",
                            "delimiter": "\""
                          }
                        }
                      }
                    },
                    "right": {
                      "IdentRef": {
                        "span": "21:22",
                        "name": "c"
                      }
                    }
                  }
                }
              }
            }
          }
        ]
      }
    },
    "test": null,
    "update": null,
    "body": {
      "Empty": {
        "span": "27:28"
      }
    }
  }
}
```
