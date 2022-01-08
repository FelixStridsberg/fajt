### Input
```js parse:expr
a?.[b]
```

### Output: ast
```json
{
  "OptionalMember": {
    "span": "0:6",
    "object": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "property": {
      "Expr": {
        "IdentRef": {
          "span": "4:5",
          "name": "b"
        }
      }
    },
    "optional": true
  }
}
```
