### Source
```js parse:expr
async function fn(param) {
    ;
}
```

### Output: minified
```js
async function fn(param){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:34",
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
      "span": "25:34",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "31:32"
          }
        }
      ]
    }
  }
}
```
