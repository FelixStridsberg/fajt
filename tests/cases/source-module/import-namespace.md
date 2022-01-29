### Source
```js source:module
import * as ns from 'module';
```

### Output: minified
```js
import*as ns from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:29",
    "directives": [],
    "body": [
      {
        "ImportDecl": {
          "span": "0:29",
          "default_binding": null,
          "namespace_binding": {
            "span": "12:14",
            "name": "ns"
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
