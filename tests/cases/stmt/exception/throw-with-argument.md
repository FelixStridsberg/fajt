### Input
```js
throw a
```

### Output: minified
```js
throw a
```

### Output: ast
```json
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
}
```
