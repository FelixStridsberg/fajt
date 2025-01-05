### Source
```js check-format:no
var a = 1; // Comment
```

### Output: minified
```js
var a=1
```

### Output: ast
```json
{
  "Script": {
    "span": "0:10",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:10",
          "kind": "Var",
          "declarations": [
            {
              "span": "4:9",
              "pattern": {
                "Ident": {
                  "span": "4:5",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "8:9",
                  "literal": {
                    "Number": {
                      "raw": "1"
                    }
                  }
                }
              }
            }
          ]
        }
      }
    ]
  }
}
```
