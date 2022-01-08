### Input
```js
async a => {}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:13",
    "asynchronous": true,
    "binding_parameter": true,
    "parameters": {
      "span": "6:7",
      "bindings": [
        {
          "span": "6:7",
          "pattern": {
            "Ident": {
              "span": "6:7",
              "name": "a"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "Body": {
        "span": "11:13",
        "directives": [],
        "statements": []
      }
    }
  }
}
```
