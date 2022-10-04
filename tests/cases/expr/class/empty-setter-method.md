### Source
```js parse:expr
class {
    set setter(a) {}
}
```

### Output: minified
```js
class{set setter(a){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:30",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:28",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "setter"
            }
          },
          "kind": "Set",
          "parameters": {
            "span": "22:25",
            "bindings": [
              {
                "span": "23:24",
                "pattern": {
                  "Ident": {
                    "span": "23:24",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "26:28",
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
