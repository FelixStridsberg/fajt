### Input
```js
try {} catch {}
```

```js min
try{}catch{}
```

### Output: ast
```json
{
  "Try": {
    "span": "0:15",
    "block": {
      "span": "4:6",
      "statements": []
    },
    "handler": {
      "span": "7:15",
      "parameter": null,
      "body": {
        "span": "13:15",
        "statements": []
      }
    },
    "finalizer": null
  }
}
```
