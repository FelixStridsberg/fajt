### Source
```js parse:expr
{ async *name() {} }
```

### Output: minified
```js
{async*name(){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:20",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:18",
              "name": {
                "Ident": {
                  "span": "9:13",
                  "name": "name"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "13:15",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "16:18",
                "directives": [],
                "statements": []
              },
              "generator": true,
              "asynchronous": true,
              "is_static": false
            }
          }
        ]
      }
    }
  }
}
```
