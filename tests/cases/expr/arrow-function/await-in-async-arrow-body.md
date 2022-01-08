### Input
```js
async () => {
  await b;
}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:26",
    "asynchronous": true,
    "binding_parameter": false,
    "parameters": {
      "span": "6:8",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Body": {
        "span": "12:26",
        "directives": [],
        "statements": [
          {
            "Expr": {
              "span": "16:24",
              "expr": {
                "Await": {
                  "span": "16:23",
                  "argument": {
                    "IdentRef": {
                      "span": "22:23",
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
}
```
