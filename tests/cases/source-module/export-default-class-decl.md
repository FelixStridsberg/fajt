### Source
```js source:module
export default class cls {}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:27",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultDecl": {
            "span": "0:27",
            "decl": {
              "Class": {
                "span": "15:27",
                "identifier": {
                  "span": "21:24",
                  "name": "cls"
                },
                "super_class": null,
                "body": []
              }
            }
          }
        }
      }
    ]
  }
}
```
