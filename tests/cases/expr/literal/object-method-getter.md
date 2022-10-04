### Source
```js parse:expr
{ get name() {} }
```

### Output: minified
```js
{get name(){}}
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
                "Ident": {
                  "span": "6:10",
                  "name": "name"
                }
              },
              "kind": "Get",
              "parameters": {
                "span": "10:12",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "13:15",
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
