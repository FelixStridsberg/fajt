### Source
```js source:module
import 'module';
import def from 'module';
import * as ns from "module";
import {} from 'module';
import def, {} from "module";
```

### Output: minified
```js
import'module';import def from'module';import*as ns from"module";import{}from'module';import def,{}from"module"
```

### Output: ast
```json
{
  "Module": {
    "span": "0:127",
    "directives": [],
    "body": [
      {
        "ImportDecl": {
          "span": "0:16",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": null,
          "from": {
            "value": "module",
            "delimiter": "'"
          }
        }
      },
      {
        "ImportDecl": {
          "span": "17:42",
          "default_binding": {
            "span": "24:27",
            "name": "def"
          },
          "namespace_binding": null,
          "named_imports": null,
          "from": {
            "value": "module",
            "delimiter": "'"
          }
        }
      },
      {
        "ImportDecl": {
          "span": "43:72",
          "default_binding": null,
          "namespace_binding": {
            "span": "55:57",
            "name": "ns"
          },
          "named_imports": null,
          "from": {
            "value": "module",
            "delimiter": "\""
          }
        }
      },
      {
        "ImportDecl": {
          "span": "73:97",
          "default_binding": null,
          "namespace_binding": null,
          "named_imports": [],
          "from": {
            "value": "module",
            "delimiter": "'"
          }
        }
      },
      {
        "ImportDecl": {
          "span": "98:127",
          "default_binding": {
            "span": "105:108",
            "name": "def"
          },
          "namespace_binding": null,
          "named_imports": [],
          "from": {
            "value": "module",
            "delimiter": "\""
          }
        }
      }
    ]
  }
}
```
