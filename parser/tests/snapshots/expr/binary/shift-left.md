```js
a << b << c
```

```json
{
  "Binary": {
    "span": "0:11",
    "operator": "ShiftLeft",
    "left": {
      "Binary": {
        "span": "0:6",
        "operator": "ShiftLeft",
        "left": {
          "IdentRef": {
            "span": "0:1",
            "name": "a"
          }
        },
        "right": {
          "IdentRef": {
            "span": "5:6",
            "name": "b"
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "10:11",
        "name": "c"
      }
    }
  }
}
```