### Source
```js parse:expr
{ a,b, c }
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:10",
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
              "span": "4:5",
              "name": "b"
            }
          },
          {
            "IdentRef": {
              "span": "7:8",
              "name": "c"
            }
          }
        ]
      }
    }
  }
}
```
