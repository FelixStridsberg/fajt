### Source
```js parse:stmt
for (a("a" in b).c in d) ;
```

### Output: minified
```js
for(a("a"in b).c in d);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:26",
    "left": {
      "Expr": {
        "Member": {
          "span": "5:18",
          "object": {
            "Expr": {
              "Call": {
                "span": "5:16",
                "callee": {
                  "Expr": {
                    "IdentRef": {
                      "span": "5:6",
                      "name": "a"
                    }
                  }
                },
                "arguments_span": "6:16",
                "arguments": [
                  {
                    "Expr": {
                      "Binary": {
                        "span": "7:15",
                        "operator": "In",
                        "left": {
                          "Literal": {
                            "span": "7:10",
                            "literal": {
                              "String": {
                                "value": "a",
                                "delimiter": "\""
                              }
                            }
                          }
                        },
                        "right": {
                          "IdentRef": {
                            "span": "14:15",
                            "name": "b"
                          }
                        }
                      }
                    }
                  }
                ]
              }
            }
          },
          "property": {
            "Ident": {
              "span": "17:18",
              "name": "c"
            }
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "22:23",
        "name": "d"
      }
    },
    "body": {
      "Empty": {
        "span": "25:26"
      }
    }
  }
}
```
