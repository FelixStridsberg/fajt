### Input
```js
export * as alias from 'other-module'
```

```json
{
  "Module": {
    "span": "0:37",
    "body": [
      {
        "ExportDeclaration": {
          "Namespace": {
            "span": "0:37",
            "alias": {
              "span": "12:17",
              "name": "alias"
            },
            "from": "other-module"
          }
        }
      }
    ]
  }
}
```
