### Source
```js parse:expr
async function* () {}
```

### Output: minified
```js
async function*(){}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:21",
    "asynchronous": true,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "16:18",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "19:21",
      "directives": [],
      "statements": []
    }
  }
}
```
