### Input
```js
a ^ b ^ c;
```

```js min
a^b^c
```

### Output: ast
```json
{
  "Binary": {
    "span": "0:9",
    "operator": "BitwiseXOR",
    "left": {
      "Binary": {
        "span": "0:5",
        "operator": "BitwiseXOR",
        "left": {
          "IdentRef": {
            "span": "0:1",
            "name": "a"
          }
        },
        "right": {
          "IdentRef": {
            "span": "4:5",
            "name": "b"
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "8:9",
        "name": "c"
      }
    }
  }
}
```
