### Source
```js source:module
export default async function* () {}
```

### Output: minified
```js
export default async function*(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:36",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultDecl": {
            "span": "0:36",
            "decl": {
              "FunctionDecl": {
                "span": "15:36",
                "asynchronous": true,
                "generator": true,
                "identifier": {
                  "span": "31:31",
                  "name": ""
                },
                "parameters": {
                  "span": "31:33",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "34:36",
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
