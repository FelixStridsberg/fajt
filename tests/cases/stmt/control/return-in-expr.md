### Source
```js parse:stmt
return "a" in b;
```

### Output: minified
```js
return"a"in b;
```

### Output: ast
```json
{
  "Return": {
    "span": "0:16",
    "argument": {
      "Binary": {
        "span": "7:15",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "7:10",
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
            "span": "14:15",
            "name": "b"
          }
        }
      }
    }
  }
}
```
