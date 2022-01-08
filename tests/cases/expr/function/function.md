### Input
```js
function fn(param) { ; }
```

```json
{
  "Function": {
    "span": "0:24",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:18",
      "bindings": [
        {
          "span": "12:17",
          "pattern": {
            "Ident": {
              "span": "12:17",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "span": "19:24",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "21:22"
          }
        }
      ]
    }
  }
}
```
