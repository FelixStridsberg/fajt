### Source
```js check-format:no
function a() {
    return
    b
}
```

### Output: minified
```js
function a(){return b}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:33",
    "directives": [],
    "body": [
      {
        "FunctionDecl": {
          "span": "0:33",
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
            "span": "13:33",
            "directives": [],
            "statements": [
              {
                "Return": {
                  "span": "19:25",
                  "argument": null
                }
              },
              {
                "Expr": {
                  "span": "30:31",
                  "expr": {
                    "IdentRef": {
                      "span": "30:31",
                      "name": "b"
                    }
                  }
                }
              }
            ]
          }
        }
      }
    ]
  }
}
```
