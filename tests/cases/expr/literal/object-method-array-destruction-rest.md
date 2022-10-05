### Source
```js parse:expr
{ method([ ...rest ]) {} }
```

### Output: minified
```js
{method([...rest]){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:26",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:24",
              "name": {
                "Ident": {
                  "span": "2:8",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:21",
                "bindings": [
                  {
                    "span": "9:20",
                    "pattern": {
                      "Array": {
                        "span": "9:20",
                        "elements": [],
                        "rest": {
                          "span": "14:18",
                          "name": "rest"
                        }
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              },
              "body": {
                "span": "22:24",
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
