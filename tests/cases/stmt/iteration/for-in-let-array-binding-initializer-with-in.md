### Source
```js parse:stmt
for ([a = b in c] in d) ;
```

### Output: minified
```js
for([a=b in c]in d);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:25",
    "left": {
      "AssignmentPattern": {
        "Array": {
          "span": "5:17",
          "elements": [
            {
              "span": "6:16",
              "target": {
                "IdentRef": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Binary": {
                  "span": "10:16",
                  "operator": "In",
                  "left": {
                    "IdentRef": {
                      "span": "10:11",
                      "name": "b"
                    }
                  },
                  "right": {
                    "IdentRef": {
                      "span": "15:16",
                      "name": "c"
                    }
                  }
                }
              }
            }
          ],
          "rest": null
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "21:22",
        "name": "d"
      }
    },
    "body": {
      "Empty": {
        "span": "24:25"
      }
    }
  }
}
```
