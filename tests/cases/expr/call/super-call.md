### Source
```js parse:expr
super();
```

### Output: minified
```js
super();
```

### Output: ast
```json
{
  "Call": {
    "span": "0:7",
    "callee": "Super",
    "arguments_span": "5:7",
    "arguments": []
  }
}
```
