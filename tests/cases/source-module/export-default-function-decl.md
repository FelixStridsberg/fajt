### Input
```js source:module
export default function fn () {}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:32",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "0:32",
            "decl": {
              "FunctionDecl": {
                "span": "15:32",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "24:26",
                  "name": "fn"
                },
                "parameters": {
                  "span": "27:29",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "30:32",
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
