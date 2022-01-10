### Source
```js parse:expr
a => {}
```

### Output: minified
```js
a=>{}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:7",
    "asynchronous": false,
    "binding_parameter": true,
    "parameters": {
      "span": "0:1",
      "bindings": [
        {
          "span": "0:1",
          "pattern": {
            "Ident": {
              "span": "0:1",
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
        "span": "5:7",
        "directives": [],
        "statements": []
      }
    }
  }
}
```