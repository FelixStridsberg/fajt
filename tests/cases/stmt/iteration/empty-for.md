### Input
```js parse:stmt
for (;;) ;
```

### Output: ast
```json
{
  "For": {
    "span": "0:10",
    "init": null,
    "test": null,
    "update": null,
    "body": {
      "Empty": {
        "span": "9:10"
      }
    }
  }
}
```
