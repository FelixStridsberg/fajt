### Source
```js parse:stmt
class cls {
    async method1() {}
}
```

### Output: minified
```js
class cls{async method1(){}}
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
          "span": "16:34",
          "name": {
            "Ident": {
              "span": "22:29",
              "name": "method1"
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
          "asynchronous": true
        }
      }
    ]
  }
}
```
