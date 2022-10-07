### Source
```js parse:stmt
for (; "a" in b; "c" in d) ;
```

### Output: minified
```js
for(;"a"in b;"c"in d);
```

### Output: ast
```json
{
  "For": {
    "span": "0:28",
    "init": null,
    "test": {
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
    "update": {
      "Binary": {
        "span": "17:25",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "17:20",
            "literal": {
              "String": {
                "value": "c",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "24:25",
            "name": "d"
          }
        }
      }
    },
    "body": {
      "Empty": {
        "span": "27:28"
      }
    }
  }
}
```
