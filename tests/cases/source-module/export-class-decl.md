### Source
```js source:module
export class cls {}
```

### Output: minified
```js
export class cls{}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:19",
    "body": [
      {
        "ExportDecl": {
          "Decl": {
            "span": "0:19",
            "decl": {
              "ClassDecl": {
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
