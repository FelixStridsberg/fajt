```js
function fn() {
    ;
    "use strict";
}
```

```js min
function fn(){;"use strict";}
```

```json
{
  "FunctionDecl": {
    "span": "0:41",
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
      "span": "14:41",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "20:21"
          }
        },
        {
          "Expr": {
            "span": "26:39",
            "expr": {
              "Literal": {
                "span": "26:38",
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
