### Input
```js parse:stmt
for (a;;) ;
```

```js
for(a;;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:11",
    "init": {
      "Expr": {
        "IdentRef": {
          "span": "5:6",
          "name": "a"
        }
      }
    },
    "test": null,
    "update": null,
    "body": {
      "Empty": {
        "span": "10:11"
      }
    }
  }
}
```
