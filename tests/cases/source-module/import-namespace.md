### Source
```js source:module
import * as ns from 'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:28",
    "body": [
      {
        "ImportDeclaration": {
          "span": "0:28",
          "default_binding": null,
          "namespace_binding": {
            "span": "12:14",
            "name": "ns"
          },
          "named_imports": null,
          "from": "module"
        }
      }
    ]
  }
}
```
