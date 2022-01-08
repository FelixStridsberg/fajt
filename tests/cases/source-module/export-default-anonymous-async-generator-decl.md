### Input
```js
export default async function*() {}
```

```json
{
  "Module": {
    "span": "0:35",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "0:35",
            "decl": {
              "FunctionDecl": {
                "span": "15:35",
                "asynchronous": true,
                "generator": true,
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
