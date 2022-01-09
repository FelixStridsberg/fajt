### Source
```js source:module
export async function fn() {}
```

### Output: minified
```js
export async function fn(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:29",
    "body": [
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "0:29",
            "decl": {
              "FunctionDecl": {
                "span": "7:29",
                "asynchronous": true,
                "generator": false,
                "identifier": {
                  "span": "22:24",
                  "name": "fn"
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
