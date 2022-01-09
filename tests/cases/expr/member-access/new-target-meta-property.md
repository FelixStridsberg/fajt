### Source
```js parse:expr
new.target
```

### Output: ast
```json
{
  "MetaProperty": {
    "span": "0:10",
    "meta": {
      "span": "0:3",
      "name": "new"
    },
    "property": {
      "span": "4:10",
      "name": "target"
    }
  }
}
```
