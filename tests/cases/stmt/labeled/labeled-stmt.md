### Source
```js parse:stmt
label: a;
```

### Output: minified
```js
label:a;
```

### Output: ast
```json
{
  "Labeled": {
    "span": "0:9",
    "label": {
      "span": "0:5",
      "name": "label"
    },
    "body": {
      "Expr": {
        "span": "7:9",
        "expr": {
          "IdentRef": {
            "span": "7:8",
            "name": "a"
          }
        }
      }
    }
  }
}
```
