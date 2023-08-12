### Source
```js parse:stmt
for (var a in b) ;
```

### Output: minified
```js
for(var a in b);
```

### Output: ast
```json
{
  "ForIn": {
    "span": "0:18",
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
      "IdentRef": {
        "span": "14:15",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "17:18"
      }
    }
  }
}
```
