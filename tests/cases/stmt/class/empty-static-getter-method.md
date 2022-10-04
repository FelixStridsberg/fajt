### Source
```js parse:stmt
class cls {
    static get getter() {}
}
```

### Output: minified
```js
class cls{static get getter(){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:40",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:38",
          "name": {
            "Ident": {
              "span": "27:33",
              "name": "getter"
            }
          },
          "kind": "Get",
          "parameters": {
            "span": "33:35",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "36:38",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": false,
          "is_static": true
        }
      }
    ]
  }
}
```
