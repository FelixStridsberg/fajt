### Source
```js parse:stmt
var {} = a;
```

### Output: minified
```js
var{}=a;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:11",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:10",
        "pattern": {
          "Object": {
            "span": "4:6",
            "props": [],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "9:10",
            "name": "a"
          }
        }
      }
    ]
  }
}
```
