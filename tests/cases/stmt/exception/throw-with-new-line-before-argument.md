### Source
```js parse:stmt check-format:no
throw
a
```

### Output: minified
```js
throw;a;
```

### Output: ast
```json
{
  "Throw": {
    "span": "0:5",
    "argument": null
  }
}
```
