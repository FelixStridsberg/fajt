```js
function fn() {
  ;
  "use strict"
}
```

```json
{
  "FunctionDecl": {
    "span": "0:36",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:13",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "14:36",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "18:19"
          }
        },
        {
          "Expr": {
            "span": "22:34",
            "expr": {
              "Literal": {
                "span": "22:34",
                "literal": {
                  "String": [
                    "use strict",
                    "\""
                  ]
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
