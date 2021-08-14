```js
async function *fn() {}
```

```json
{
  "FunctionDecl": {
    "span": "0:23",
    "asynchronous": true,
    "generator": true,
    "identifier": {
      "span": "16:18",
      "name": "fn"
    },
    "parameters": {
      "span": "18:20",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "21:23",
      "directives": [],
      "statements": []
    }
  }
}
```
