### Input
```js source:module
export class cls {}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:19",
    "body": [
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "0:19",
            "decl": {
              "Class": {
                "span": "7:19",
                "identifier": {
                  "span": "13:16",
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
