### Input
```js
import {} from 'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:23",
    "body": [
      {
        "ImportDeclaration": {
          "span": "0:23",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": [],
          "from": "module"
        }
      }
    ]
  }
}
```
