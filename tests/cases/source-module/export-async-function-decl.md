### Input
```js
export async function fn () {}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:30",
    "body": [
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "0:30",
            "decl": {
              "FunctionDecl": {
                "span": "7:30",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "22:24",
                  "name": "fn"
                },
                "parameters": {
                  "span": "25:27",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "28:30",
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
