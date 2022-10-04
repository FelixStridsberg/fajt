### Source
```js parse:stmt
class cls {
    static set setter(a) {}
}
```

### Output: minified
```js
class cls{static set setter(a){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:41",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "23:39",
          "name": {
            "Ident": {
              "span": "27:33",
              "name": "setter"
            }
          },
          "kind": "Set",
          "parameters": {
            "span": "33:36",
            "bindings": [
              {
                "span": "34:35",
                "pattern": {
                  "Ident": {
                    "span": "34:35",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "37:39",
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
