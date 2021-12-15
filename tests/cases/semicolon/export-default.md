```js
export default a + b
export default a + b;
export default function () {}
export default function () {};
export default function fn() {}
export default function fn() {};
export default function*() {}
export default function*() {};
export default function* fn() {}
export default function* fn() {};
export default class {}
export default class {};
export default class cls {}
export default class cls {};
```

```json
{
  "Module": {
    "span": "0:402",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultExpr": {
            "span": "0:20",
            "expr": {
              "Binary": {
                "span": "15:20",
                "operator": "Plus",
                "left": {
                  "IdentRef": {
                    "span": "15:16",
                    "name": "a"
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
        }
      },
      {
        "ExportDeclaration": {
          "DefaultExpr": {
            "span": "21:42",
            "expr": {
              "Binary": {
                "span": "36:41",
                "operator": "Plus",
                "left": {
                  "IdentRef": {
                    "span": "36:37",
                    "name": "a"
                  }
                },
                "right": {
                  "IdentRef": {
                    "span": "40:41",
                    "name": "b"
                  }
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "43:72",
            "decl": {
              "FunctionDecl": {
                "span": "58:72",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "67:67",
                  "name": ""
                },
                "parameters": {
                  "span": "67:69",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "70:72",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "73:102",
            "decl": {
              "FunctionDecl": {
                "span": "88:102",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "97:97",
                  "name": ""
                },
                "parameters": {
                  "span": "97:99",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "100:102",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "102:103"
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "104:135",
            "decl": {
              "FunctionDecl": {
                "span": "119:135",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "128:130",
                  "name": "fn"
                },
                "parameters": {
                  "span": "130:132",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "133:135",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "136:167",
            "decl": {
              "FunctionDecl": {
                "span": "151:167",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "160:162",
                  "name": "fn"
                },
                "parameters": {
                  "span": "162:164",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "165:167",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "167:168"
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "169:198",
            "decl": {
              "FunctionDecl": {
                "span": "184:198",
                "asynchronous": false,
                "generator": true,
                "identifier": {
                  "span": "193:193",
                  "name": ""
                },
                "parameters": {
                  "span": "193:195",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "196:198",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "199:228",
            "decl": {
              "FunctionDecl": {
                "span": "214:228",
                "asynchronous": false,
                "generator": true,
                "identifier": {
                  "span": "223:223",
                  "name": ""
                },
                "parameters": {
                  "span": "223:225",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "226:228",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "228:229"
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "230:262",
            "decl": {
              "FunctionDecl": {
                "span": "245:262",
                "asynchronous": false,
                "generator": true,
                "identifier": {
                  "span": "255:257",
                  "name": "fn"
                },
                "parameters": {
                  "span": "257:259",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "260:262",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "263:295",
            "decl": {
              "FunctionDecl": {
                "span": "278:295",
                "asynchronous": false,
                "generator": true,
                "identifier": {
                  "span": "288:290",
                  "name": "fn"
                },
                "parameters": {
                  "span": "290:292",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "293:295",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "295:296"
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "297:320",
            "decl": {
              "Class": {
                "span": "312:320",
                "identifier": {
                  "span": "318:318",
                  "name": ""
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "321:344",
            "decl": {
              "Class": {
                "span": "336:344",
                "identifier": {
                  "span": "342:342",
                  "name": ""
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "344:345"
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "346:373",
            "decl": {
              "Class": {
                "span": "361:373",
                "identifier": {
                  "span": "367:370",
                  "name": "cls"
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      },
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "374:401",
            "decl": {
              "Class": {
                "span": "389:401",
                "identifier": {
                  "span": "395:398",
                  "name": "cls"
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      },
      {
        "Empty": {
          "span": "401:402"
        }
      }
    ]
  }
}
```
