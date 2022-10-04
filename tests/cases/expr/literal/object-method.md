### Source
```js parse:expr
{ name() {} }
```

### Output: minified
```js
{name(){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:13",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:11",
              "name": {
                "Ident": {
                  "span": "2:6",
                  "name": "name"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "6:8",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "9:11",
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
