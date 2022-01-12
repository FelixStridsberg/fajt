### Source
```js source:module
export function fn() {}
```

### Output: minified
```js
export function fn(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:23",
    "body": [
      {
        "ExportDecl": {
          "Decl": {
            "span": "0:23",
            "decl": {
              "FunctionDecl": {
                "span": "7:23",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "16:18",
                  "name": "fn"
                },
                "parameters": {
                  "span": "18:20",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "21:23",
                  "directives": [],
                  "statements": []
                }
              }
            }
          }
        }
      }
    ]
  }
}
```
