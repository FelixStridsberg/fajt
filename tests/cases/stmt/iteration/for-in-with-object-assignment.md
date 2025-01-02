### Source
```js parse:stmt
for ({ a } in d) ;
```

### Output: minified
```js
for({a,b:c=1}in d);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:18",
    "left": {
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
    }
  }
}
```
