### Source
```js parse:stmt
for (a["a" in b] in c) ;
```

### Output: minified
```js
for(a["a"in b]in c);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:24",
    "left": {
      "Expr": {
        "Member": {
          "span": "5:16",
          "object": {
            "Expr": {
              "IdentRef": {
                "span": "5:6",
                "name": "a"
              }
            }
          },
          "property": {
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
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "20:21",
        "name": "c"
      }
    },
    "body": {
      "Empty": {
        "span": "23:24"
      }
    }
  }
}
```
