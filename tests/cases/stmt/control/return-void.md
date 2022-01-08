### Input
```js parse:stmt
return;
```

### Output: minified
```js
return
```

### Output: ast
```json
{
  "Return": {
    "span": "0:7",
    "argument": null
  }
}
```
