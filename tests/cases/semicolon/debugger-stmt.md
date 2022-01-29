### Source
```js check-format:no
debugger
debugger;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:18",
    "directives": [],
    "body": [
      {
        "Debugger": {
          "span": "0:8"
        }
      },
      {
        "Debugger": {
          "span": "9:18"
        }
      }
    ]
  }
}
```
