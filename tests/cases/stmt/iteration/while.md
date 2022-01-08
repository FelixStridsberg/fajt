### Input
```js
while (true) a;
```

```js min
while(true)a
```

```json
{
  "While": {
    "span": "0:15",
    "test": {
      "Literal": {
        "span": "7:11",
        "literal": {
          "Boolean": true
        }
      }
    },
    "body": {
      "Expr": {
        "span": "13:15",
        "expr": {
          "IdentRef": {
            "span": "13:14",
            "name": "a"
          }
        }
      }
    }
  }
}
```
