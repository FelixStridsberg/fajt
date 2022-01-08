### Input
```js
function* fn() {
    yield;
    a;
}
```

```js min
function*fn(){yield;a}
```

```json
{
  "FunctionDecl": {
    "span": "0:36",
    "asynchronous": false,
    "generator": true,
    "identifier": {
      "span": "10:12",
      "name": "fn"
    },
    "parameters": {
      "span": "12:14",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "15:36",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "21:27",
            "expr": {
              "Yield": {
                "span": "21:26",
                "argument": null,
                "delegate": false
              }
            }
          }
        },
        {
          "Expr": {
            "span": "32:34",
            "expr": {
              "IdentRef": {
                "span": "32:33",
                "name": "a"
              }
            }
          }
        }
      ]
    }
  }
}
```
