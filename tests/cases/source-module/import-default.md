### Source
```js source:module
import abc from 'module';
```

### Output: minified
```js
import abc from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:25",
    "body": [
      {
        "ImportDecl": {
          "span": "0:25",
          "default_binding": {
            "span": "7:10",
            "name": "abc"
          },
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
