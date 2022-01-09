### Source
```js parse:stmt
var { ...rest } = c;
```

### Output: minified
```js
var{...rest}=c;
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:20",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:19",
        "pattern": {
          "Object": {
            "span": "4:15",
            "props": [],
            "rest": {
              "span": "9:13",
              "name": "rest"
            }
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "18:19",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
