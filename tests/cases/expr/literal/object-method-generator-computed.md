### Source
```js parse:expr
{ *[name](v) {} }
```

### Output: minified
```js
{*[name](v){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:17",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:15",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "4:8",
                    "name": "name"
                  }
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "9:12",
                "bindings": [
                  {
                    "span": "10:11",
                    "pattern": {
                      "Ident": {
                        "span": "10:11",
                        "name": "v"
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              },
              "body": {
                "span": "13:15",
                "directives": [],
                "statements": []
              },
              "generator": true,
              "asynchronous": false,
              "is_static": false
            }
          }
        ]
      }
    }
  }
}
```
