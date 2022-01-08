### Input
```js
for (var a in b) ;
```

```js min
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
        "declarations": [
          {
            "span": "9:10",
            "pattern": {
              "Ident": {
                "span": "9:10",
                "name": "a"
              }
            },
            "initializer": null
          }
        ]
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
