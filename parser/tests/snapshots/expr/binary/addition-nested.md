```js
a + b + c + d
```

```json
{
  "Binary": {
    "span": "0:13",
    "operator": "Plus",
    "left": {
      "Binary": {
        "span": "0:9",
        "operator": "Plus",
        "left": {
          "Binary": {
            "span": "0:5",
            "operator": "Plus",
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
    },
    "right": {
      "IdentRef": {
        "span": "12:13",
        "name": "d"
      }
    }
  }
}
```
