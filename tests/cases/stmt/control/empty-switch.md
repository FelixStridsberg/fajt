### Input
```js
switch (a) {}
```

### Output: minified
```js
switch(a){}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:13",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": []
  }
}
```
