### Input
```js parse:stmt
for (var a;;) ;
```

### Output: minified
```js
for(var a;;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:15",
    "init": {
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
    "test": null,
    "update": null,
    "body": {
      "Empty": {
        "span": "14:15"
      }
    }
  }
}
```
