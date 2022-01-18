### Source
```js
`this is template string`;
```

### Output: minified
```js
`this is template string`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:26",
    "body": [
      {
        "Expr": {
          "span": "0:26",
          "expr": {
            "Literal": {
              "span": "0:25",
              "literal": {
                "Template": {
                  "parts": [
                    {
                      "String": "this is template string"
                    }
                  ]
                }
              }
            }
          }
        }
      }
    ]
  }
}
```
