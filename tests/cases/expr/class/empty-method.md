### Source
```js parse:expr
class {
    method1() {}
}
```

### Output: minified
```js
class{method1(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:26",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:24",
          "name": {
            "Ident": {
              "span": "12:19",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "19:21",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "22:24",
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
