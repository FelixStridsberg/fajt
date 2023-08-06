### Source
```js parse:expr
class {
    static() {}
}
```

### Output: minified
```js
class{static(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:25",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:23",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "static"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:20",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "21:23",
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
