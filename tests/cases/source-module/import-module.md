### Source
```js source:module
import 'module';
```

### Output: minified
```js
import'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:16",
    "directives": [],
    "body": [
      {
        "ImportDecl": {
          "span": "0:16",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": null,
          "from": {
            "value": "module",
            "delimiter": "'"
          }
        }
      }
    ]
  }
}
```
