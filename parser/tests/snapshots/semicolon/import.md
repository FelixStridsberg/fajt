```js
import 'module';
import def from 'module';
import * as ns from 'module';
import {} from 'module';
import def, {} from 'module';
```

```json
{
  "Module": {
    "span": "0:127",
    "body": [
      {
        "ImportDeclaration": {
          "span": "0:16",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": null,
          "source": "module"
        }
      },
      {
        "ImportDeclaration": {
          "span": "17:42",
          "default_binding": {
            "span": "24:27",
            "name": "def"
          },
          "namespace_binding": null,
          "named_imports": null,
          "source": "module"
        }
      },
      {
        "ImportDeclaration": {
          "span": "43:72",
          "default_binding": null,
          "namespace_binding": {
            "span": "55:57",
            "name": "ns"
          },
          "named_imports": null,
          "source": "module"
        }
      },
      {
        "ImportDeclaration": {
          "span": "73:97",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": [],
          "source": "module"
        }
      },
      {
        "ImportDeclaration": {
          "span": "98:127",
          "default_binding": {
            "span": "105:108",
            "name": "def"
          },
          "namespace_binding": null,
          "named_imports": [],
          "source": "module"
        }
      }
    ]
  }
}
```
