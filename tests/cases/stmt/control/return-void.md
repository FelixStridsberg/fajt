### Source
```js parse:stmt
function a() {
    return;
}
```

### Output: minified
```js
function a(){return;}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:28",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:10",
      "name": "a"
    },
    "parameters": {
      "span": "10:12",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "13:28",
      "directives": [],
      "statements": [
        {
          "Return": {
            "span": "19:26",
            "argument": null
          }
        }
      ]
    }
  }
}
```
