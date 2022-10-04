### Source
```js parse:stmt
class cls {
    static method1() {}
}
```

### Output: minified
```js
class cls{static method1(){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:37",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:35",
          "name": {
            "Ident": {
              "span": "23:30",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "30:32",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "33:35",
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
