```js
a * b * c
```

```json
{
  "Binary": {
    "span": "0:9",
    "operator": "Multiplication",
    "left": {
      "Binary": {
        "span": "0:5",
        "operator": "Multiplication",
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