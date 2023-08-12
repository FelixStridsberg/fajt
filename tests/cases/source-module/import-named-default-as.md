### Source
```js source:module
import { default as a } from 'module';
```

### Output: minified
```js
import{default as a}from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:38",
    "directives": [],
    "body": [
      {
        "ImportDecl": {
          "span": "0:38",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": [
            {
              "span": "9:21",
              "name": {
                "span": "9:16",
                "name": "default"
              },
              "alias": {
                "span": "20:21",
                "name": "a"
              }
            }
          ],
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
