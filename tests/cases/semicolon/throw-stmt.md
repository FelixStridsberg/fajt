### Source
```js check-format:no
throw a
throw a;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:16",
    "directives": [],
    "body": [
      {
        "Throw": {
          "span": "0:7",
          "argument": {
            "IdentRef": {
              "span": "6:7",
              "name": "a"
            }
          }
        }
      },
      {
        "Throw": {
          "span": "8:16",
          "argument": {
            "IdentRef": {
              "span": "14:15",
              "name": "a"
            }
          }
        }
      }
    ]
  }
}
```
