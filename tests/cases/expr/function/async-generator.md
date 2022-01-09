### Source
```js parse:expr
async function* fn(param) { ; }
```

### Output: ast
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
    "body": {
      "span": "26:31",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "28:29"
          }
        }
      ]
    }
  }
}
```
