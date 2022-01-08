### Input
```js
a !== b !== c;
```

```js min
a!==b!==c
```

```json
{
  "Binary": {
    "span": "0:13",
    "operator": "StrictNotEqual",
    "left": {
      "Binary": {
        "span": "0:7",
        "operator": "StrictNotEqual",
        "left": {
          "IdentRef": {
            "span": "0:1",
            "name": "a"
          }
        },
        "right": {
          "IdentRef": {
            "span": "6:7",
            "name": "b"
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "12:13",
        "name": "c"
      }
    }
  }
}
```
