### Source
```js parse:expr
class {
    static async method1() {}
}
```

### Output: minified
```js
class{static async method1(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:39",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "19:37",
          "name": {
            "Ident": {
              "span": "25:32",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "32:34",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "35:37",
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
