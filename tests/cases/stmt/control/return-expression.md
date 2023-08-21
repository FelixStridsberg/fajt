### Source
```js parse:stmt
function a() {
    return b;
}
```

### Output: minified
```js
function a(){return b}

```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:30",
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
      "span": "13:30",
      "directives": [],
      "statements": [
        {
          "Return": {
            "span": "19:28",
            "argument": {
              "IdentRef": {
                "span": "26:27",
                "name": "b"
              }
            }
          }
        }
      ]
    }
  }
}
```
