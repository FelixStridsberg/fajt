### Source
```js parse:expr
async () => {
    await b;
}
```

### Output: minified
```js
async()=>{await b}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:28",
    "asynchronous": true,
    "binding_parameter": false,
    "parameters": {
      "span": "6:8",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Body": {
        "span": "12:28",
        "directives": [],
        "statements": [
          {
            "Expr": {
              "span": "18:26",
              "expr": {
                "Await": {
                  "span": "18:25",
                  "argument": {
                    "IdentRef": {
                      "span": "24:25",
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