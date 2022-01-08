### Input
```js
var [ a, b, c ] = d;
```

### Output: minified
```js min
var[a,b,c]=d
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
          "Array": {
            "span": "4:15",
            "elements": [
              {
                "span": "6:7",
                "pattern": {
                  "Ident": {
                    "span": "6:7",
                    "name": "a"
                  }
                },
                "initializer": null
              },
              {
                "span": "9:10",
                "pattern": {
                  "Ident": {
                    "span": "9:10",
                    "name": "b"
                  }
                },
                "initializer": null
              },
              {
                "span": "12:13",
                "pattern": {
                  "Ident": {
                    "span": "12:13",
                    "name": "c"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          }
        },
        "initializer": {
          "IdentRef": {
            "span": "18:19",
            "name": "d"
          }
        }
      }
    ]
  }
}
```
