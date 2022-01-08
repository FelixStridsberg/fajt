### Input
```js
for (let a;;) ;
```

```js min
for(let a;;);
```

### Output: ast
```json
{
  "For": {
    "span": "0:15",
    "init": {
      "Declaration": {
        "span": "5:10",
        "kind": "Let",
        "declarations": [
          {
            "span": "9:10",
            "pattern": {
              "Ident": {
                "span": "9:10",
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
        "span": "14:15"
      }
    }
  }
}
```
