### Source
```js parse:stmt
try {} finally {}
```

### Output: minified
```js
try{}finally{}
```

### Output: ast
```json
{
  "Try": {
    "span": "0:17",
    "block": {
      "span": "4:6",
      "statements": []
    },
    "handler": null,
    "finalizer": {
      "span": "15:17",
      "statements": []
    }
  }
}
```
