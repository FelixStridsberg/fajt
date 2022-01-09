### Source
```js source:module
import def, * as a from 'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:32",
    "body": [
      {
        "ImportDeclaration": {
          "span": "0:32",
          "default_binding": {
            "span": "7:10",
            "name": "def"
          },
          "namespace_binding": {
            "span": "17:18",
            "name": "a"
          },
          "named_imports": null,
          "from": "module"
        }
      }
    ]
  }
}
```
