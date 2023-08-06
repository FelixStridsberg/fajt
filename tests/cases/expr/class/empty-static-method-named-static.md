### Source
```js parse:expr
class {
    static static() {}
}
```

### Output: minified
```js
class{static static(){}}
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
          "span": "19:30",
          "name": {
            "Ident": {
              "span": "19:25",
              "name": "static"
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
          "asynchronous": false,
          "is_static": true
        }
      }
    ]
  }
}
```
