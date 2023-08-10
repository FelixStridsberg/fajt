### Source
```js parse:stmt
for ({ a } of d) ;
```

### Output: minified
```js
for({a,b:c=1}of d);
```

### Output: ast
```json
{
  "ForOf": {
    "span": "0:18",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Object": {
            "span": "5:10",
            "props": [
              {
                "Single": {
                  "span": "7:8",
                  "ident": {
                    "span": "7:8",
                    "name": "a"
                  },
                  "initializer": null
                }
              }
            ],
            "rest": null
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "14:15",
        "name": "d"
      }
    },
    "body": {
      "Empty": {
        "span": "17:18"
      }
    },
    "asynchronous": false
  }
}
```
