### Source
```js parse:expr
function fn(param) {
    ;
}
```

### Output: minified
```js
function fn(param){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:28",
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
      "span": "19:28",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "25:26"
          }
        }
      ]
    }
  }
}
```
