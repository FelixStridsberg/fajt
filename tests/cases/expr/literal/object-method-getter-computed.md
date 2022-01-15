### Source
```js parse:expr
{ get[name]() {} }
```

### Output: minified
```js
{get[name](){}}
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
                "Computed": {
                  "IdentRef": {
                    "span": "6:10",
                    "name": "name"
                  }
                }
              },
              "kind": "Get",
              "parameters": {
                "span": "11:13",
                "bindings": [],
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
