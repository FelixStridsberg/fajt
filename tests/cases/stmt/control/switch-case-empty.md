### Input
```js
switch (a) {
    case b:
}
```

### Output: minified
```js
switch(a){case b:}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:26",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "17:24",
        "test": {
          "IdentRef": {
            "span": "22:23",
            "name": "b"
          }
        },
        "consequent": []
      }
    ]
  }
}
```
