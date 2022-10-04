### Source
```js parse:expr
class {
    static method1() {}
}
```

### Output: minified
```js
class{static method1(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:33",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "19:31",
          "name": {
            "Ident": {
              "span": "19:26",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "26:28",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "29:31",
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
