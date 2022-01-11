### Source
```js parse:expr
{ a, b, c }
```

### Output: minified
```js
{a,b,c}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:11",
    "literal": {
      "Object": {
        "props": [
          {
            "IdentRef": {
              "span": "2:3",
              "name": "a"
            }
          },
          {
            "IdentRef": {
              "span": "5:6",
              "name": "b"
            }
          },
          {
            "IdentRef": {
              "span": "8:9",
              "name": "c"
            }
          }
        ]
      }
    }
  }
}
```
