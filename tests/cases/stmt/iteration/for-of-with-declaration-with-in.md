### Source
```js parse:stmt
for (var a of "b" in c) ;
```

### Output: minified
```js
for(var a of"b"in c);
```

### Output: ast
```json
{
  "ForOf": {
    "span": "0:25",
    "left": {
      "Declaration": {
        "span": "5:10",
        "kind": "Var",
        "binding": {
          "Ident": {
            "span": "9:10",
            "name": "a"
          }
        }
      }
    },
    "right": {
      "Binary": {
        "span": "14:22",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "14:17",
            "literal": {
              "String": {
                "value": "b",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "21:22",
            "name": "c"
          }
        }
      }
    },
    "body": {
      "Empty": {
        "span": "24:25"
      }
    },
    "asynchronous": false
  }
}
```
