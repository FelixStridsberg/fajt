```js
export default async function fn () {}
```

```json
{
  "Module": {
    "span": "0:38",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "0:38",
            "decl": {
              "FunctionDecl": {
                "span": "15:38",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "30:32",
                  "name": "fn"
                },
                "parameters": {
                  "span": "33:35",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "36:38",
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
