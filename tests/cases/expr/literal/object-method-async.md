### Source
```js parse:expr
{ async name() {} }
```

### Output: minified
```js
{async name(){}}
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
                "Ident": {
                  "span": "8:12",
                  "name": "name"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "12:14",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "15:17",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": true
            }
          }
        ]
      }
    }
  }
}
```
