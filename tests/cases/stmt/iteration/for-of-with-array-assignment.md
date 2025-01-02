### Source
```js parse:stmt
for ([ a ] of b) ;
```

### Output: minified
```js
for([a]of b);
```

### Output: ast
```json
{
  "ForOf": {
    "span": "0:18",
    "left": {
      "AssignmentPattern": {
        "Array": {
          "span": "5:10",
          "elements": [
            {
              "span": "7:8",
              "target": {
                "Expr": {
                  "IdentRef": {
                    "span": "7:8",
                    "name": "a"
                  }
                }
              },
              "initializer": null
            }
          ],
          "rest": null
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
    },
    "asynchronous": false
  }
}
```
