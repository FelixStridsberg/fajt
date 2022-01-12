### Source
```js parse:expr
(a) => {
    ;
}
```

### Output: minified
```js
(a)=>{}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:16",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:3",
      "bindings": [
        {
          "span": "1:2",
          "pattern": {
            "Ident": {
              "span": "1:2",
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
        "span": "7:16",
        "directives": [],
        "statements": [
          {
            "Empty": {
              "span": "13:14"
            }
          }
        ]
      }
    }
  }
}
```
