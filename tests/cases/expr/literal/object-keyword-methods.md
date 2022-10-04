### Source
```js parse:expr
{ get() {}, set() {} }
```

### Output: minified
```js
{get(){},set(){}}
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
              "span": "2:10",
              "name": {
                "Ident": {
                  "span": "2:5",
                  "name": "get"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "5:7",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "8:10",
                "directives": [],
                "statements": []
              },
              "generator": false,
              "asynchronous": false
            }
          },
          {
            "Method": {
              "span": "12:20",
              "name": {
                "Ident": {
                  "span": "12:15",
                  "name": "set"
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
