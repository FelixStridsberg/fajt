### Source
```js parse:stmt
for (;; a) ;
```

### Output: minified
```js
for(;;a);
```

### Output: ast
```json
{
  "For": {
    "span": "0:12",
    "init": null,
    "test": null,
    "update": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "body": {
      "Empty": {
        "span": "11:12"
      }
    }
  }
}
```
