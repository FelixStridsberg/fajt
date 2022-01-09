### Source
```js parse:stmt
switch (a) {
    default:
}
```

### Output: minified
```js
switch(a){default:}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:27",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "17:25",
        "test": null,
        "consequent": []
      }
    ]
  }
}
```
