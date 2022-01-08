### Input
```js parse:stmt
async function fn() {
    for await (a of b) ;
}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:48",
    "asynchronous": true,
    "generator": false,
    "identifier": {
      "span": "15:17",
      "name": "fn"
    },
    "parameters": {
      "span": "17:19",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "20:48",
      "directives": [],
      "statements": [
        {
          "ForOf": {
            "span": "26:46",
            "left": {
              "Expr": {
                "IdentRef": {
                  "span": "37:38",
                  "name": "a"
                }
              }
            },
            "right": {
              "IdentRef": {
                "span": "42:43",
                "name": "b"
              }
            },
            "body": {
              "Empty": {
                "span": "45:46"
              }
            },
            "asynchronous": true
          }
        }
      ]
    }
  }
}
```
