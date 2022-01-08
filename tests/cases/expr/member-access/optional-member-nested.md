### Input
```js
a?.b?.c?.d
```

```json
{
  "OptionalMember": {
    "span": "0:10",
    "object": {
      "OptionalMember": {
        "span": "0:7",
        "object": {
          "OptionalMember": {
            "span": "0:4",
            "object": {
              "IdentRef": {
                "span": "0:1",
                "name": "a"
              }
            },
            "property": {
              "Ident": {
                "span": "3:4",
                "name": "b"
              }
            },
            "optional": true
          }
        },
        "property": {
          "Ident": {
            "span": "6:7",
            "name": "c"
          }
        },
        "optional": true
      }
    },
    "property": {
      "Ident": {
        "span": "9:10",
        "name": "d"
      }
    },
    "optional": true
  }
}
```
