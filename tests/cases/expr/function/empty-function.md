### Source
```js parse:expr
function fn() {}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:16",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:13",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "14:16",
      "directives": [],
      "statements": []
    }
  }
}
```
