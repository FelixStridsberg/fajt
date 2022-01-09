### Source
```js parse:expr
async function* (param) {
    ;
}
```

### Output: minified
```js
async function*(param){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:33",
    "asynchronous": true,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "16:23",
      "bindings": [
        {
          "span": "17:22",
          "pattern": {
            "Ident": {
              "span": "17:22",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "span": "24:33",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "30:31"
          }
        }
      ]
    }
  }
}
```
