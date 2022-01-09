### Source
```js source:module
export default function () {}
```

### Output: minified
```js
export default function(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:29",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "0:29",
            "decl": {
              "FunctionDecl": {
                "span": "15:29",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "24:24",
                  "name": ""
                },
                "parameters": {
                  "span": "24:26",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "27:29",
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
