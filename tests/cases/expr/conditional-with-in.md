### Source
```js parse:expr
("a" in b) ? "c" in d : "e" in f
```

### Output: minified
```js
("a"in b)?"c"in d:"e"in f
```

### Output: ast
```json
{
  "Conditional": {
    "span": "0:32",
    "condition": {
      "Parenthesized": {
        "span": "0:10",
        "expression": {
          "Binary": {
            "span": "1:9",
            "operator": "In",
            "left": {
              "Literal": {
                "span": "1:4",
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
                "span": "8:9",
                "name": "b"
              }
            }
          }
        }
      }
    },
    "consequent": {
      "Binary": {
        "span": "13:21",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "13:16",
            "literal": {
              "String": {
                "value": "c",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "20:21",
            "name": "d"
          }
        }
      }
    },
    "alternate": {
      "Binary": {
        "span": "24:32",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "24:27",
            "literal": {
              "String": {
                "value": "e",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "31:32",
            "name": "f"
          }
        }
      }
    }
  }
}
```
