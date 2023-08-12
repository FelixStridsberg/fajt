### Source
```js source:module
export * as default from 'other-module'
```

### Output: minified
```js
export*as default from'other-module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:39",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "Namespace": {
            "span": "0:39",
            "alias": {
              "span": "12:19",
              "name": "default"
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
