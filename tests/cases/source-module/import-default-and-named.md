### Source
```js source:module
import def, { a, b } from 'module';
```

### Output: minified
```js
import def,{a,b}from'module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:35",
    "body": [
      {
        "ImportDecl": {
          "span": "0:35",
          "default_binding": {
            "span": "7:10",
            "name": "def"
          },
          "namespace_binding": null,
          "named_imports": [
            {
              "span": "14:15",
              "name": {
                "span": "14:15",
                "name": "a"
              },
              "alias": null
            },
            {
              "span": "17:18",
              "name": {
                "span": "17:18",
                "name": "b"
              },
              "alias": null
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
