### Source
```js parse:stmt
class cls {
    static() {}
}
```

### Output: minified
```js
class cls{static(){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:29",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:27",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "static"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:24",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "25:27",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": false,
          "is_static": false
        }
      }
    ]
  }
}
```
