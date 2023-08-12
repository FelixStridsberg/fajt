### Source
```js parse:stmt
for (let {a = 1 in b, c: d = e in f} in d) ;
```

### Output: minified
```js
for(let{a=1 in b,c:d=e in f}in d);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:44",
    "left": {
      "Declaration": {
        "span": "5:36",
        "kind": "Let",
        "declarations": [
          {
            "span": "9:36",
            "pattern": {
              "Object": {
                "span": "9:36",
                "props": [
                  {
                    "Single": {
                      "span": "10:20",
                      "ident": {
                        "span": "10:11",
                        "name": "a"
                      },
                      "initializer": {
                        "Binary": {
                          "span": "14:20",
                          "operator": "In",
                          "left": {
                            "Literal": {
                              "span": "14:15",
                              "literal": {
                                "Number": {
                                  "Integer": [
                                    1,
                                    "Decimal"
                                  ]
                                }
                              }
                            }
                          },
                          "right": {
                            "IdentRef": {
                              "span": "19:20",
                              "name": "b"
                            }
                          }
                        }
                      }
                    }
                  },
                  {
                    "Named": {
                      "span": "22:35",
                      "property": {
                        "Ident": {
                          "span": "22:23",
                          "name": "c"
                        }
                      },
                      "binding": {
                        "span": "25:35",
                        "pattern": {
                          "Ident": {
                            "span": "25:26",
                            "name": "d"
                          }
                        },
                        "initializer": {
                          "Binary": {
                            "span": "29:35",
                            "operator": "In",
                            "left": {
                              "IdentRef": {
                                "span": "29:30",
                                "name": "e"
                              }
                            },
                            "right": {
                              "IdentRef": {
                                "span": "34:35",
                                "name": "f"
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                ],
                "rest": null
              }
            },
            "initializer": null
          }
        ]
      }
    },
    "right": {
      "IdentRef": {
        "span": "40:41",
        "name": "d"
      }
    },
    "body": {
      "Empty": {
        "span": "43:44"
      }
    }
  }
}
```
