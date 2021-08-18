```js
async function fn() {
  for await (a of b) ;
}
```

```json
{
  "FunctionDecl": {
    "span": "0:46",
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
      "span": "20:46",
      "directives": [],
      "statements": [
        {
          "ForOf": {
            "span": "24:44",
            "left": {
              "Expr": {
                "IdentRef": {
                  "span": "35:36",
                  "name": "a"
                }
              }
            },
            "right": {
              "IdentRef": {
                "span": "40:41",
                "name": "b"
              }
            },
            "body": {
              "Empty": {
                "span": "43:44"
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
