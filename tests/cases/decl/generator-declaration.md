### Input
```js parse:stmt
function* fn() {}
```

### Output: minified
```js
function*fn(){}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:17",
    "asynchronous": false,
    "generator": true,
    "identifier": {
      "span": "10:12",
      "name": "fn"
    },
    "parameters": {
      "span": "12:14",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "15:17",
      "directives": [],
      "statements": []
    }
  }
}
```
