### Source
```js parse:expr
a
++
```

// TODO this should probably fail on the ++?
### Output: ast
```json
{
  "IdentRef": {
    "span": "0:1",
    "name": "a"
  }
}
```
