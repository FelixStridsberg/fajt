### Source
```js parse:expr
{ [name](v) {} }
```

### Output: minified
```js
{[name](v){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:16",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:14",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "3:7",
                    "name": "name"
                  }
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:11",
                "bindings": [
                  {
                    "span": "9:10",
                    "pattern": {
                      "Ident": {
                        "span": "9:10",
                        "name": "v"
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              },
              "body": {
                "span": "12:14",
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
  }
}
```
