### Source
```js parse:stmt check-format:no
return
a
```

### Output: minified
```js
return;a;
```

### Output: ast
```json
{
  "Return": {
    "span": "0:6",
    "argument": null
  }
}
```
