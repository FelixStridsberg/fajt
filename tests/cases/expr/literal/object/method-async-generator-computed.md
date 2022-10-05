### Source
```js parse:expr
{ async *[name]() {} }
```

### Output: minified
```js
{async*[name](){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:22",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:20",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "10:14",
                    "name": "name"
                  }
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "15:17",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "18:20",
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
