### Source
```js source:module
import def, * as a from 'module';
```

### Output: minified
```js
import def,*as a from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:33",
    "body": [
      {
        "ImportDecl": {
          "span": "0:33",
          "default_binding": {
            "span": "7:10",
            "name": "def"
          },
          "namespace_binding": {
            "span": "17:18",
            "name": "a"
          },
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
