### Input
```js
var { a = b } = c;
```

### Output: minified
```js min
var{a=b}=c
```

### Output: ast
```json
{
  "Variable": {
    "span": "0:18",
    "kind": "Var",
    "declarations": [
      {
        "span": "4:17",
        "pattern": {
          "Object": {
            "span": "4:13",
            "props": [
              {
                "Single": [
                  {
                    "span": "6:7",
                    "name": "a"
                  },
                  {
                    "IdentRef": {
                      "span": "10:11",
                      "name": "b"
                    }
                  }
                ]
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "16:17",
            "name": "c"
          }
        }
      }
    ]
  }
}
```
