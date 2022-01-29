### Source
```js
async();
async(a, b);
```

### Output: minified
```js
async();async(a,b);
```

### Output: ast
```json
{
  "Script": {
    "span": "0:21",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:8",
          "expr": {
            "Call": {
              "span": "0:7",
              "callee": {
                "Expr": {
                  "IdentRef": {
                    "span": "0:5",
                    "name": "async"
                  }
                }
              },
              "arguments_span": "5:7",
              "arguments": []
            }
          }
        }
      },
      {
        "Expr": {
          "span": "9:21",
          "expr": {
            "Call": {
              "span": "9:20",
              "callee": {
                "Expr": {
                  "IdentRef": {
                    "span": "9:14",
                    "name": "async"
                  }
                }
              },
              "arguments_span": "14:20",
              "arguments": [
                {
                  "Expr": {
                    "IdentRef": {
                      "span": "15:16",
                      "name": "a"
                    }
                  }
                },
                {
                  "Expr": {
                    "IdentRef": {
                      "span": "18:19",
                      "name": "b"
                    }
                  }
                }
              ]
            }
          }
        }
      }
    ]
  }
}
```
