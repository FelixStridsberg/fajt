### Source
```js source:module
export default async function () {}
```

### Output: minified
```js
export default async function(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:35",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultDecl": {
            "span": "0:35",
            "decl": {
              "FunctionDecl": {
                "span": "15:35",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "30:30",
                  "name": ""
                },
                "parameters": {
                  "span": "30:32",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "33:35",
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
