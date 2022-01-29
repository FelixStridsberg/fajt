### Source
```js source:module
export default function fn() {}
```

### Output: minified
```js
export default function fn(){}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:31",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultDecl": {
            "span": "0:31",
            "decl": {
              "FunctionDecl": {
                "span": "15:31",
                "asynchronous": false,
                "generator": false,
                "identifier": {
                  "span": "24:26",
                  "name": "fn"
                },
                "parameters": {
                  "span": "26:28",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "29:31",
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
