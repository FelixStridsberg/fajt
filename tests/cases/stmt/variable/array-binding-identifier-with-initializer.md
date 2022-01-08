### Input
```js
var [ a = b ] = c;
```

```js min
var[a=b]=c
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
          "Array": {
            "span": "4:13",
            "elements": [
              {
                "span": "6:11",
                "pattern": {
                  "Ident": {
                    "span": "6:7",
                    "name": "a"
                  }
                },
                "initializer": {
                  "IdentRef": {
                    "span": "10:11",
                    "name": "b"
                  }
                }
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
