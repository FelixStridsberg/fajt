### Source
```js parse:expr
{ *name() {} }
```

### Output: minified
```js
{*name(){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:14",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:12",
              "name": {
                "Ident": {
                  "span": "3:7",
                  "name": "name"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "7:9",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "10:12",
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
