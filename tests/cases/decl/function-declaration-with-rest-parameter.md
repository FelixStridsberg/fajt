### Source
```js parse:stmt
function fn(...a) {}
```

### Output: minified
```js
function fn(...a){}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:20",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:17",
      "bindings": [],
      "rest": {
        "Ident": {
          "span": "15:16",
          "name": "a"
        }
      }
    },
    "body": {
      "span": "18:20",
      "directives": [],
      "statements": []
    }
  }
}
```
