### Source
```js source:module
export default class {}
```

### Output: minified
```js
export default class{}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:23",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultDecl": {
            "span": "0:23",
            "decl": {
              "ClassDecl": {
                "span": "15:23",
                "identifier": {
                  "span": "21:21",
                  "name": ""
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
