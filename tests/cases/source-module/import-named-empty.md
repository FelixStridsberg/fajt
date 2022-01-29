### Source
```js source:module
import {} from 'module';
```

### Output: minified
```js
import{}from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:24",
    "directives": [],
    "body": [
      {
        "ImportDecl": {
          "span": "0:24",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": [],
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
