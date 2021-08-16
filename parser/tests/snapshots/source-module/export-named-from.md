```js
export { a, b as c } from "other-module"
```

```json
{
  "Module": {
    "span": "0:40",
    "body": [
      {
        "ExportDeclaration": {
          "Named": {
            "span": "0:40",
            "named_exports": [
              {
                "span": "9:10",
                "name": {
                  "span": "9:10",
                  "name": "a"
                },
                "alias_of": null
              },
              {
                "span": "12:18",
                "name": {
                  "span": "17:18",
                  "name": "c"
                },
                "alias_of": {
                  "span": "12:13",
                  "name": "b"
                }
              }
            ],
            "from": "other-module"
          }
        }
      }
    ]
  }
}
```
