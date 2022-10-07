### Source
```js parse:stmt
for (; a;) ;
```

### Output: minified
```js
for(;a;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:12",
    "init": null,
    "test": {
      "IdentRef": {
        "span": "7:8",
        "name": "a"
      }
    },
    "update": null,
    "body": {
      "Empty": {
        "span": "11:12"
      }
    }
  }
}
```
