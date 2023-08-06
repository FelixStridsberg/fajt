### Source
```js parse:stmt
class cls {
    method([ ...rest ]) {}
}
```

### Output: minified
```js
class cls{method([...rest]){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:40",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:38",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:35",
            "bindings": [
              {
                "span": "23:34",
                "pattern": {
                  "Array": {
                    "span": "23:34",
                    "elements": [],
                    "rest": {
                      "Ident": {
                        "span": "28:32",
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
            "span": "36:38",
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
