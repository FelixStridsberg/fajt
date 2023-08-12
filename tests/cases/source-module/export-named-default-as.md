### Source
```js source:module
export { default as a };
```

### Output: minified
```js
export{default as a}
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
                  "span": "20:21",
                  "name": "a"
                },
                "alias_of": {
                  "span": "9:16",
                  "name": "default"
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
