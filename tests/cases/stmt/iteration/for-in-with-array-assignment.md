### Source
```js parse:stmt
for ([ a ] in b) ;
```

### Output: minified
```js
for([a]in b);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:18",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Array": {
            "span": "5:10",
            "elements": [
              {
                "span": "7:8",
                "target": {
                  "IdentRef": {
                    "span": "7:8",
                    "name": "a"
                  }
                },
                "initializer": null
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
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "17:18"
      }
    }
  }
}
```
