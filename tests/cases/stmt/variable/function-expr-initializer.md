### Source
```js
const fn = function () {
    ;
};
```

### Output: minified
```js
const fn=function(){}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:33",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:33",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:32",
              "pattern": {
                "Ident": {
                  "span": "6:8",
                  "name": "fn"
                }
              },
              "initializer": {
                "Function": {
                  "span": "11:32",
                  "asynchronous": false,
                  "generator": false,
                  "identifier": null,
                  "parameters": {
                    "span": "20:22",
                    "bindings": [],
                    "rest": null
                  },
                  "body": {
                    "span": "23:32",
                    "directives": [],
                    "statements": [
                      {
                        "Empty": {
                          "span": "29:30"
                        }
                      }
                    ]
                  }
                }
              }
            }
          ]
        }
      }
    ]
  }
}
```
