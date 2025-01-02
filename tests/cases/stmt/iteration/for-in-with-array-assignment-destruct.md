### Source
```js parse:stmt
for ([ { a } ] in b) ;
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:22",
    "left": {
      "AssignmentPattern": {
        "Array": {
          "span": "5:14",
          "elements": [
            {
              "span": "7:12",
              "target": {
                "AssignmentPattern": {
                  "Object": {
                    "span": "7:12",
                    "props": [
                      {
                        "Single": {
                          "span": "9:10",
                          "ident": {
                            "span": "9:10",
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
              "initializer": null
            }
          ],
          "rest": null
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "18:19",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "21:22"
      }
    }
  }
}
```
