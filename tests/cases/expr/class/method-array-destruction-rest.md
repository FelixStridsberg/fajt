### Source
```js parse:expr
class {
    method([ ...rest ]) {}
}
```

### Output: minified
```js
class{method([...rest]){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:36",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:34",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:31",
            "bindings": [
              {
                "span": "19:30",
                "pattern": {
                  "Array": {
                    "span": "19:30",
                    "elements": [],
                    "rest": {
                      "Ident": {
                        "span": "24:28",
                        "name": "rest"
                      }
                    }
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "32:34",
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
