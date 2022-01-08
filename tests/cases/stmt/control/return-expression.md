### Input
```js
return a;
```

```js min
return a
```

### Output: ast
```json
{
  "Return": {
    "span": "0:9",
    "argument": {
      "IdentRef": {
        "span": "7:8",
        "name": "a"
      }
    }
  }
}
```
