### Input
```js
a instanceof b instanceof c;
```

```js min
a instanceof b instanceof c
```

### Output: ast
```json
{
  "Binary": {
    "span": "0:27",
    "operator": "InstanceOf",
    "left": {
      "Binary": {
        "span": "0:14",
        "operator": "InstanceOf",
        "left": {
          "IdentRef": {
            "span": "0:1",
            "name": "a"
          }
        },
        "right": {
          "IdentRef": {
            "span": "13:14",
            "name": "b"
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "26:27",
        "name": "c"
      }
    }
  }
}
```
