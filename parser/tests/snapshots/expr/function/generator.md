```js
function *(param) { ; }
```

```json
{
  "Function": {
    "span": "0:23",
    "asynchronous": false,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "10:17",
      "bindings": [
        {
          "span": "11:16",
          "pattern": {
            "Ident": {
              "span": "11:16",
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
          "span": "20:21"
        }
      }
    ]
  }
}
```