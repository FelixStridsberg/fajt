### Source
```js parse:expr
test ? "a" in b : alternate
```

### Output: minified
```js
test?"a"in b:alternate
```

### Output: ast
```json
{
  "Conditional": {
    "span": "0:27",
    "condition": {
      "IdentRef": {
        "span": "0:4",
        "name": "test"
      }
    },
    "consequent": {
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
    },
    "alternate": {
      "IdentRef": {
        "span": "18:27",
        "name": "alternate"
      }
    }
  }
}
```
