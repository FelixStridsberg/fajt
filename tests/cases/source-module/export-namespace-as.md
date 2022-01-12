### Source
```js source:module
export * as alias from 'other-module'
```

### Output: minified
```js
export*as alias from'other-module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:37",
    "body": [
      {
        "ExportDecl": {
          "Namespace": {
            "span": "0:37",
            "alias": {
              "span": "12:17",
              "name": "alias"
            },
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
