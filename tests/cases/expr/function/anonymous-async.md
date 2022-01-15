### Source
```js parse:expr
async function (param) {
    ;
}
```

### Output: minified
```js
async function(param){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:32",
    "asynchronous": true,
    "generator": false,
    "identifier": null,
    "parameters": {
      "span": "15:22",
      "bindings": [
        {
          "span": "16:21",
          "pattern": {
            "Ident": {
              "span": "16:21",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "span": "23:32",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "29:30"
          }
        }
      ]
    }
  }
}
```
