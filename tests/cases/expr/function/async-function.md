### Input
```js
async function fn(param) { ; }
```

```json
{
  "Function": {
    "span": "0:30",
    "asynchronous": true,
    "generator": false,
    "identifier": {
      "span": "15:17",
      "name": "fn"
    },
    "parameters": {
      "span": "17:24",
      "bindings": [
        {
          "span": "18:23",
          "pattern": {
            "Ident": {
              "span": "18:23",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "span": "25:30",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "27:28"
          }
        }
      ]
    }
  }
}
```
