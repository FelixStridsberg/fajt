### Source
```js parse:stmt
class cls {
    static async method1() {}
}
```

### Output: minified
```js
class cls{static async method1(){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:43",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:41",
          "name": {
            "Ident": {
              "span": "29:36",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "36:38",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "39:41",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": true,
          "is_static": true
        }
      }
    ]
  }
}
```
