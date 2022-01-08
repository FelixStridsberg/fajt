### Input
```js
for (;;a) ;
```

### Output: minified
```js min
for(;;a);
```

### Output: ast
```json
{
  "For": {
    "span": "0:11",
    "init": null,
    "test": null,
    "update": {
      "IdentRef": {
        "span": "7:8",
        "name": "a"
      }
    },
    "body": {
      "Empty": {
        "span": "10:11"
      }
    }
  }
}
```
