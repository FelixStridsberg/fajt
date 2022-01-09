### Source
```js parse:expr
async function fn() {}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:22",
    "asynchronous": true,
    "generator": false,
    "identifier": {
      "span": "15:17",
      "name": "fn"
    },
    "parameters": {
      "span": "17:19",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "20:22",
      "directives": [],
      "statements": []
    }
  }
}
```
