### Source
```js check-format:no
/* inline */ throw /* inline */ "error";
```

### Output: minified
```js
throw"error"
```

### Output: ast
```json
{
  "Script": {
    "span": "13:40",
    "directives": [],
    "body": [
      {
        "Throw": {
          "span": "13:40",
          "argument": {
            "Literal": {
              "span": "32:39",
              "literal": {
                "String": {
                  "value": "error",
                  "delimiter": "\""
                }
              }
            }
          }
        }
      }
    ]
  }
}
```
