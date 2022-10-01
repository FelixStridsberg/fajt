### Source
```js
a.await;
a.break;
a.case;
a.catch;
a.class;
a.const;
a.continue;
a.debugger;
a.default;
a.delete;
a.do;
a.else;
a.enum;
a.export;
a.extends;
a.false;
a.finally;
a.for;
a.function;
a.if;
a.import;
a.in;
a.instanceof;
a.new;
a.null;
a.return;
a.super;
a.switch;
a.this;
a.throw;
a.true;
a.try;
a.typeof;
a.var;
a.void;
a.while;
a.with;
a.yield;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:342",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:8",
          "expr": {
            "Member": {
              "span": "0:7",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "0:1",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "2:7",
                  "name": "await"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "9:17",
          "expr": {
            "Member": {
              "span": "9:16",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "9:10",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "11:16",
                  "name": "break"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "18:25",
          "expr": {
            "Member": {
              "span": "18:24",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "18:19",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "20:24",
                  "name": "case"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "26:34",
          "expr": {
            "Member": {
              "span": "26:33",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "26:27",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "28:33",
                  "name": "catch"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "35:43",
          "expr": {
            "Member": {
              "span": "35:42",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "35:36",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "37:42",
                  "name": "class"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "44:52",
          "expr": {
            "Member": {
              "span": "44:51",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "44:45",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "46:51",
                  "name": "const"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "53:64",
          "expr": {
            "Member": {
              "span": "53:63",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "53:54",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "55:63",
                  "name": "continue"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "65:76",
          "expr": {
            "Member": {
              "span": "65:75",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "65:66",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "67:75",
                  "name": "debugger"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "77:87",
          "expr": {
            "Member": {
              "span": "77:86",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "77:78",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "79:86",
                  "name": "default"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "88:97",
          "expr": {
            "Member": {
              "span": "88:96",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "88:89",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "90:96",
                  "name": "delete"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "98:103",
          "expr": {
            "Member": {
              "span": "98:102",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "98:99",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "100:102",
                  "name": "do"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "104:111",
          "expr": {
            "Member": {
              "span": "104:110",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "104:105",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "106:110",
                  "name": "else"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "112:119",
          "expr": {
            "Member": {
              "span": "112:118",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "112:113",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "114:118",
                  "name": "enum"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "120:129",
          "expr": {
            "Member": {
              "span": "120:128",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "120:121",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "122:128",
                  "name": "export"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "130:140",
          "expr": {
            "Member": {
              "span": "130:139",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "130:131",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "132:139",
                  "name": "extends"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "141:149",
          "expr": {
            "Member": {
              "span": "141:148",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "141:142",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "143:148",
                  "name": "false"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "150:160",
          "expr": {
            "Member": {
              "span": "150:159",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "150:151",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "152:159",
                  "name": "finally"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "161:167",
          "expr": {
            "Member": {
              "span": "161:166",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "161:162",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "163:166",
                  "name": "for"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "168:179",
          "expr": {
            "Member": {
              "span": "168:178",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "168:169",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "170:178",
                  "name": "function"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "180:185",
          "expr": {
            "Member": {
              "span": "180:184",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "180:181",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "182:184",
                  "name": "if"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "186:195",
          "expr": {
            "Member": {
              "span": "186:194",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "186:187",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "188:194",
                  "name": "import"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "196:201",
          "expr": {
            "Member": {
              "span": "196:200",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "196:197",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "198:200",
                  "name": "in"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "202:215",
          "expr": {
            "Member": {
              "span": "202:214",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "202:203",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "204:214",
                  "name": "instanceof"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "216:222",
          "expr": {
            "Member": {
              "span": "216:221",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "216:217",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "218:221",
                  "name": "new"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "223:230",
          "expr": {
            "Member": {
              "span": "223:229",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "223:224",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "225:229",
                  "name": "null"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "231:240",
          "expr": {
            "Member": {
              "span": "231:239",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "231:232",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "233:239",
                  "name": "return"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "241:249",
          "expr": {
            "Member": {
              "span": "241:248",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "241:242",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "243:248",
                  "name": "super"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "250:259",
          "expr": {
            "Member": {
              "span": "250:258",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "250:251",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "252:258",
                  "name": "switch"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "260:267",
          "expr": {
            "Member": {
              "span": "260:266",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "260:261",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "262:266",
                  "name": "this"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "268:276",
          "expr": {
            "Member": {
              "span": "268:275",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "268:269",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "270:275",
                  "name": "throw"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "277:284",
          "expr": {
            "Member": {
              "span": "277:283",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "277:278",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "279:283",
                  "name": "true"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "285:291",
          "expr": {
            "Member": {
              "span": "285:290",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "285:286",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "287:290",
                  "name": "try"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "292:301",
          "expr": {
            "Member": {
              "span": "292:300",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "292:293",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "294:300",
                  "name": "typeof"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "302:308",
          "expr": {
            "Member": {
              "span": "302:307",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "302:303",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "304:307",
                  "name": "var"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "309:316",
          "expr": {
            "Member": {
              "span": "309:315",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "309:310",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "311:315",
                  "name": "void"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "317:325",
          "expr": {
            "Member": {
              "span": "317:324",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "317:318",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "319:324",
                  "name": "while"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "326:333",
          "expr": {
            "Member": {
              "span": "326:332",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "326:327",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "328:332",
                  "name": "with"
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "334:342",
          "expr": {
            "Member": {
              "span": "334:341",
              "object": {
                "Expr": {
                  "IdentRef": {
                    "span": "334:335",
                    "name": "a"
                  }
                }
              },
              "property": {
                "Ident": {
                  "span": "336:341",
                  "name": "yield"
                }
              }
            }
          }
        }
      }
    ]
  }
}
```
