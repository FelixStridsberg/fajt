### Input
```js
export function fn () {}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:24",
    "body": [
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "0:24",
            "decl": {
              "FunctionDecl": {
                "span": "7:24",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "16:18",
                  "name": "fn"
                },
                "parameters": {
                  "span": "19:21",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "22:24",
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
