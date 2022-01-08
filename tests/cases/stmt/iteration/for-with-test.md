### Input
```js
for (;a;) ;
```

```js min
for(;a;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:11",
    "init": null,
    "test": {
      "IdentRef": {
        "span": "6:7",
        "name": "a"
      }
    },
    "update": null,
    "body": {
      "Empty": {
        "span": "10:11"
      }
    }
  }
}
```
