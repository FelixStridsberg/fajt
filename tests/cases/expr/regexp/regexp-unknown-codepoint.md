### Source
Regexp may contain code points that are not recognized by the parser.
This catches that edge case.

```js check-format:no
/#/;
/@/;
/☂/;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:16",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:4",
          "expr": {
            "Literal": {
              "span": "0:3",
              "literal": {
                "Regexp": "/#/"
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "5:9",
          "expr": {
            "Literal": {
              "span": "5:8",
              "literal": {
                "Regexp": "/@/"
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "10:16",
          "expr": {
            "Literal": {
              "span": "10:15",
              "literal": {
                "Regexp": "/☂/"
              }
            }
          }
        }
      }
    ]
  }
}
```
