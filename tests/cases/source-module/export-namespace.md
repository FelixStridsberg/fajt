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
    "body": [
      {
        "ExportDeclaration": {
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
