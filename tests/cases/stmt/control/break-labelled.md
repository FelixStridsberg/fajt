### Source
```js parse:stmt
break a;
```

### Output: minified
```js
break a;
```

### Output: ast
```json
{
  "Break": {
    "span": "0:8",
    "label": {
      "span": "6:7",
      "name": "a"
    }
  }
}
```
