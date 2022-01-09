### Source
```js parse:expr
async function () {}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:20",
    "asynchronous": true,
    "generator": false,
    "identifier": null,
    "parameters": {
      "span": "15:17",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "18:20",
      "directives": [],
      "statements": []
    }
  }
}
```
