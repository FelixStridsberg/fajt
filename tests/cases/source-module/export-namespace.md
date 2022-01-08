### Input
```js
export * from 'other-module'
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
            "from": "other-module"
          }
        }
      }
    ]
  }
}
```
