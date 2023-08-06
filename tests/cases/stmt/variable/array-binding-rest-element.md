### Source
```js parse:stmt
var [ ...a ] = b;
```

### Output: minified
```js
var[...a]=b;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:17",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:16",
        "pattern": {
          "Array": {
            "span": "4:12",
            "elements": [],
            "rest": {
              "Ident": {
                "span": "9:10",
                "name": "a"
              }
            }
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "15:16",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
