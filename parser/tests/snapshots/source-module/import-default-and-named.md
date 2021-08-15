```js
import def, {a, b} from 'module'
```

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
          "namespace_binding": null,
          "named_imports": [
            {
              "span": "13:14",
              "name": {
                "span": "13:14",
                "name": "a"
              },
              "alias": null
            },
            {
              "span": "16:17",
              "name": {
                "span": "16:17",
                "name": "b"
              },
              "alias": null
            }
          ],
          "source": "module"
        }
      }
    ]
  }
}
```
