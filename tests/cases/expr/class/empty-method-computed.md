### Source
```js parse:expr
class {
    [name]() {}
}
```

### Output: minified
```js
class{[name](){}}
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
            "Computed": {
              "IdentRef": {
                "span": "13:17",
                "name": "name"
              }
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
