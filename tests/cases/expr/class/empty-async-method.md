### Source
```js parse:expr
class {
    async method1() {}
}
```

### Output: minified
```js
class{async method1(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:32",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:30",
          "name": {
            "Ident": {
              "span": "18:25",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "25:27",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "28:30",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": true,
          "is_static": false
        }
      }
    ]
  }
}
```
