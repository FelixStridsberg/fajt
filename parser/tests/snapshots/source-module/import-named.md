```js
import { a, b as c } from 'module'
```

```json
{
  "Module": {
    "span": "0:34",
    "body": [
      {
        "ImportDeclaration": {
          "span": "0:34",
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
          "source": "module"
        }
      }
    ]
  }
}
```
