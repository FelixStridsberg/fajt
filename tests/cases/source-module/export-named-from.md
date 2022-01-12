### Source
```js source:module
export { a, b as c } from 'other-module';
```

### Output: minified
```js
export{a,b as c}from'other-module'
```

### Output: ast
```json
{
  "Module": {
    "span": "0:41",
    "body": [
      {
        "ExportDecl": {
          "Named": {
            "span": "0:41",
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
            "from": {
              "value": "other-module",
              "delimiter": "'"
            }
          }
        }
      }
    ]
  }
}
```
