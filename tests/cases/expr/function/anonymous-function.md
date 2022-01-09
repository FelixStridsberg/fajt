### Source
```js parse:expr
function (param) {
    ;
}
```

### Output: minified
```js
function(param){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:26",
    "asynchronous": false,
    "generator": false,
    "identifier": null,
    "parameters": {
      "span": "9:16",
      "bindings": [
        {
          "span": "10:15",
          "pattern": {
            "Ident": {
              "span": "10:15",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "span": "17:26",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "23:24"
          }
        }
      ]
    }
  }
}
```
