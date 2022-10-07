### Source
```js parse:stmt
for (a in "b" in c) ;
```

### Output: minified
```js
for(a in"b"in c);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:21",
    "left": {
      "Expr": {
        "IdentRef": {
          "span": "5:6",
          "name": "a"
        }
      }
    },
    "right": {
      "Binary": {
        "span": "10:18",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "10:13",
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
            "span": "17:18",
            "name": "c"
          }
        }
      }
    },
    "body": {
      "Empty": {
        "span": "20:21"
      }
    }
  }
}
```
