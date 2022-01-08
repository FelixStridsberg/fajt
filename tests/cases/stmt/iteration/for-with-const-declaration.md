### Input
```js
for (const a;;) ;
```

```js min
for(const a;;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:17",
    "init": {
      "Declaration": {
        "span": "5:12",
        "kind": "Const",
        "declarations": [
          {
            "span": "11:12",
            "pattern": {
              "Ident": {
                "span": "11:12",
                "name": "a"
              }
            },
            "initializer": null
          }
        ]
      }
    },
    "test": null,
    "update": null,
    "body": {
      "Empty": {
        "span": "16:17"
      }
    }
  }
}
```
