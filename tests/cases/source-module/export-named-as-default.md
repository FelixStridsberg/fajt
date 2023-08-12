### Source
```js source:module
export { a as default };
```

### Output: minified
```js
export{a as default}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:24",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "Named": {
            "span": "0:24",
            "named_exports": [
              {
                "span": "9:21",
                "name": {
                  "span": "14:21",
                  "name": "default"
                },
                "alias_of": {
                  "span": "9:10",
                  "name": "a"
                }
              }
            ],
            "from": null
          }
        }
      }
    ]
  }
}
```
