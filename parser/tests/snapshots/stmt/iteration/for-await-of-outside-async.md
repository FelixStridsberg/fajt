```js
function fn() {
  for await (a of b) ;
}
```

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
