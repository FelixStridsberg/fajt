### Input
```js
var [ , ] = b;
```

```js min
var[,]=b
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:14",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:13",
        "pattern": {
          "Array": {
            "span": "4:9",
            "elements": [
              null
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "12:13",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
