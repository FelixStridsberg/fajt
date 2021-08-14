```js
function fn(a, b = c) { }
```

```json
{
  "FunctionDecl": {
    "span": "0:25",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:21",
      "bindings": [
        {
          "span": "12:13",
          "pattern": {
            "Ident": {
              "span": "12:13",
              "name": "a"
            }
          },
          "initializer": null
        },
        {
          "span": "15:20",
          "pattern": {
            "Ident": {
              "span": "15:16",
              "name": "b"
            }
          },
          "initializer": {
            "IdentRef": {
              "span": "19:20",
              "name": "c"
            }
          }
        }
      ],
      "rest": null
    },
    "body": {
      "span": "22:25",
      "directives": [],
      "statements": []
    }
  }
}
```
