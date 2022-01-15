### Source
```js parse:expr
{ set[name](v) {} }
```

### Output: minified
```js
{set[name](v){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:19",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:17",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "6:10",
                    "name": "name"
                  }
                }
              },
              "kind": "Set",
              "parameters": {
                "span": "11:14",
                "bindings": [
                  {
                    "span": "12:13",
                    "pattern": {
                      "Ident": {
                        "span": "12:13",
                        "name": "v"
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              },
              "body": {
                "span": "15:17",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          }
        ]
      }
    }
  }
}
```
