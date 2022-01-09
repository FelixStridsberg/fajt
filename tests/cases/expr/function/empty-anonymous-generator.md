### Source
```js parse:expr
function* () {}
```

### Output: minified
```js
function*(){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:15",
    "asynchronous": false,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "10:12",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "13:15",
      "directives": [],
      "statements": []
    }
  }
}
```
