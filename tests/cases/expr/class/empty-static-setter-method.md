### Source
```js parse:expr
class {
    static set setter(a) {}
}
```

### Output: minified
```js
class{static set setter(a){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:37",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "19:35",
          "name": {
            "Ident": {
              "span": "23:29",
              "name": "setter"
            }
          },
          "kind": "Set",
          "parameters": {
            "span": "29:32",
            "bindings": [
              {
                "span": "30:31",
                "pattern": {
                  "Ident": {
                    "span": "30:31",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "33:35",
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
