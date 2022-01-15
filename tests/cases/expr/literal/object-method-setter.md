### Source
```js parse:expr
{ set name(v) {} }
```

### Output: minified
```js
{set name(v){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:18",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:16",
              "name": {
                "Ident": {
                  "span": "6:10",
                  "name": "name"
                }
              },
              "kind": "Set",
              "parameters": {
                "span": "10:13",
                "bindings": [
                  {
                    "span": "11:12",
                    "pattern": {
                      "Ident": {
                        "span": "11:12",
                        "name": "v"
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              },
              "body": {
                "span": "14:16",
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
