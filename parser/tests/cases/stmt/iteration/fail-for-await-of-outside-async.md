### Input
```js parse:stmt
function fn() {
  for await (a of b) ;
}
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Await"
    },
    "first_on_line": false,
    "span": "22:27"
  }
}
```
