### Source
```js parse:stmt
class cls {
    static static() {}
}
```

### Output: minified
```js
class cls{static static(){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:36",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:34",
          "name": {
            "Ident": {
              "span": "23:29",
              "name": "static"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "29:31",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "32:34",
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
