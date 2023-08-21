### Source
```js parse:stmt
function a() {
    return "a" in b;
}
```

### Output: minified
```js
function(){return "a" in b;}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:37",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:10",
      "name": "a"
    },
    "parameters": {
      "span": "10:12",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "13:37",
      "directives": [],
      "statements": [
        {
          "Return": {
            "span": "19:35",
            "argument": {
              "Binary": {
                "span": "26:34",
                "operator": "In",
                "left": {
                  "Literal": {
                    "span": "26:29",
                    "literal": {
                      "String": {
                        "value": "a",
                        "delimiter": "\""
                      }
                    }
                  }
                },
                "right": {
                  "IdentRef": {
                    "span": "33:34",
                    "name": "b"
                  }
                }
              }
            }
          }
        }
      ]
    }
  }
}
```
