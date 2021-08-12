```js
async function* fn(param) { ; }
```

```json
{
  "Function": {
    "span": "0:31",
    "asynchronous": true,
    "generator": true,
    "identifier": {
      "span": "16:18",
      "name": "fn"
    },
    "parameters": {
      "span": "18:25",
      "bindings": [
        {
          "span": "19:24",
          "pattern": {
            "Ident": {
              "span": "19:24",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": [
      {
        "Empty": {
          "span": "28:29"
        }
      }
    ]
  }
}
```
