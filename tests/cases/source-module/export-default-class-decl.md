### Source
```js source:module
export default class cls {}
```

### Output: minified
```js
export default class cls{}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:27",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultDecl": {
            "span": "0:27",
            "decl": {
              "ClassDecl": {
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
