### Input
```js parse:stmt
async function fn() { var await = 1 }
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Await"
    },
    "first_on_line": false,
    "span": "26:31"
  }
}
```
