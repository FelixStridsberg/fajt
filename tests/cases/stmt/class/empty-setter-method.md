### Source
```js parse:stmt
class cls {
    set setter(a) {}
}
```

### Output: minified
```js
class cls{set setter(a){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:34",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:32",
          "name": {
            "Ident": {
              "span": "20:26",
              "name": "setter"
            }
          },
          "kind": "Set",
          "parameters": {
            "span": "26:29",
            "bindings": [
              {
                "span": "27:28",
                "pattern": {
                  "Ident": {
                    "span": "27:28",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "30:32",
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
