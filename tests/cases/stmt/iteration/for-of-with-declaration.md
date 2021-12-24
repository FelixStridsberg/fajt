```js
for (var a of b) ;
```

```js min
for(var a of b);
```

```json
{
  "ForOf": {
    "span": "0:18",
    "left": {
      "Declaration": {
        "span": "5:10",
        "kind": "Var",
        "declarations": [
          {
            "span": "9:10",
            "pattern": {
              "Ident": {
                "span": "9:10",
                "name": "a"
              }
            },
            "initializer": null
          }
        ]
      }
    },
    "right": {
      "IdentRef": {
        "span": "14:15",
        "name": "b"
      }
    },
    "body": {
      "Empty": {
        "span": "17:18"
      }
    },
    "asynchronous": false
  }
}
```
