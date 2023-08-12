### Source
```js source:module
export { default };
```

### Output: minified
```js
export{default}
```

### Output: ast
```json
{
  "Module": {
    "span": "0:19",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "Named": {
            "span": "0:19",
            "named_exports": [
              {
                "span": "9:16",
                "name": {
                  "span": "9:16",
                  "name": "default"
                },
                "alias_of": null
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
