```js
a ?? b ?? c
```

```json
{
  "Logical": {
    "span": "0:11",
    "operator": "Coalesce",
    "left": {
      "Logical": {
        "span": "0:6",
        "operator": "Coalesce",
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
