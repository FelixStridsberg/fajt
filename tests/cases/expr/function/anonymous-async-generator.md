### Input
```js parse:expr
async function *(param) { ; }
```

### Output: ast
```json
{
  "Function": {
    "span": "0:29",
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
      "span": "24:29",
      "directives": [],
      "statements": [
        {
          "Empty": {
            "span": "26:27"
          }
        }
      ]
    }
  }
}
```
