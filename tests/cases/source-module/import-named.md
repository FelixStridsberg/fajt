### Source
```js source:module
import { a, b as c } from 'module';
```

### Output: minified
```js
import{a,b as c}from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:35",
    "directives": [],
    "body": [
      {
        "ImportDecl": {
          "span": "0:35",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": [
            {
              "span": "9:10",
              "name": {
                "span": "9:10",
                "name": "a"
              },
              "alias": null
            },
            {
              "span": "12:18",
              "name": {
                "span": "12:13",
                "name": "b"
              },
              "alias": {
                "span": "17:18",
                "name": "c"
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
