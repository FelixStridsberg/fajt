### Source
```js parse:stmt
if ("a" in b)
    ;
```

### Output: minified
```js
if("a"in b);
```

### Output: ast
```json
{
  "If": {
    "span": "0:19",
    "condition": {
      "Binary": {
        "span": "4:12",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "4:7",
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
            "span": "11:12",
            "name": "b"
          }
        }
      }
    },
    "consequent": {
      "Empty": {
        "span": "18:19"
      }
    },
    "alternate": null
  }
}
```
