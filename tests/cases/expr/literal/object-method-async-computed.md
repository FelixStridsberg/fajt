### Source
```js parse:expr
{ async [name]() {} }
```

### Output: minified
```js
{async[name](){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:21",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:19",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "9:13",
                    "name": "name"
                  }
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "14:16",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "17:19",
                "directives": [],
                "statements": []
              },
              "generator": false,
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
