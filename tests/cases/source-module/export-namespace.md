### Source
```js source:module
export * from 'other-module'
```

### Output: minified
```js
export*from'other-module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:28",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "Namespace": {
            "span": "0:28",
            "alias": null,
            "from": {
              "value": "other-module",
              "delimiter": "'"
            }
          }
        }
      }
    ]
  }
}
```
