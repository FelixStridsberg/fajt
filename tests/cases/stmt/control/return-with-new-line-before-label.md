### Source
```js parse:stmt check-format:no
break
a
```

### Output: minified
```js
break;a;
```

### Output: ast
```json
{
  "Break": {
    "span": "0:5",
    "label": null
  }
}
```
