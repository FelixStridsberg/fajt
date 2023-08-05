### Source
```js parse:expr check-format:no
class {
    ;method1() {};
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
    "span": "0:28",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "13:25",
          "name": {
            "Ident": {
              "span": "13:20",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "20:22",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "23:25",
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
