### Input
```js
class cls {
    method1() {}
}
```

### Output: minified
```js
class cls{method1(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:30",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:28",
          "name": {
            "Ident": {
              "span": "16:23",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "23:25",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "26:28",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": false
        }
      }
    ]
  }
}
```
