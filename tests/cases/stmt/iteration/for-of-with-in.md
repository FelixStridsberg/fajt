### Source
```js parse:stmt
for (a of "b" in c) ;
```

### Output: minified
```js
for(a of"b"in c);
```

### Output: ast
```json
{
  "ForOf": {
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
    },
    "asynchronous": false
  }
}
```
