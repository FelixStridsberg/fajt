### Input
```js
fn().a;
```

```js min
fn().a
```

```json
{
  "Member": {
    "span": "0:6",
    "object": {
      "Expr": {
        "Call": {
          "span": "0:4",
          "callee": {
            "Expr": {
              "IdentRef": {
                "span": "0:2",
                "name": "fn"
              }
            }
          },
          "arguments_span": "2:4",
          "arguments": []
        }
      }
    },
    "property": {
      "Ident": {
        "span": "5:6",
        "name": "a"
      }
    }
  }
}
```
