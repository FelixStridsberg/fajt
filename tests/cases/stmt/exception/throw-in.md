### Source
```js parse:stmt
throw "a" in b
```

### Output: minified
```js
throw"a"in b
```

### Output: ast
```json
{
  "Throw": {
    "span": "0:14",
    "argument": {
      "Binary": {
        "span": "6:14",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "6:9",
            "literal": {
              "String": {
                "value": "a",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "13:14",
            "name": "b"
          }
        }
      }
    }
  }
}
```
