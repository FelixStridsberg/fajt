### Source
```js parse:stmt
continue a;
```

### Output: minified
```js
continue a;
```

### Output: ast
```json
{
  "Continue": {
    "span": "0:11",
    "label": {
      "span": "9:10",
      "name": "a"
    }
  }
}
```
