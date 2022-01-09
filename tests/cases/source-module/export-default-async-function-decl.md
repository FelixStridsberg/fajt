### Source
```js source:module
export default async function fn() {}
```

### Output: minified
```js
export default async function fn(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:37",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "0:37",
            "decl": {
              "FunctionDecl": {
                "span": "15:37",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "30:32",
                  "name": "fn"
                },
                "parameters": {
                  "span": "32:34",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "35:37",
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
