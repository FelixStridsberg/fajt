### Source
```js check-format:no
function a() {
    return
    return;
}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:39",
    "directives": [],
    "body": [
      {
        "FunctionDecl": {
          "span": "0:39",
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
            "span": "13:39",
            "directives": [],
            "statements": [
              {
                "Return": {
                  "span": "19:25",
                  "argument": null
                }
              },
              {
                "Return": {
                  "span": "30:37",
                  "argument": null
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
