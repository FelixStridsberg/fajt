```js
yield
* a
```

// TODO does not parse the * a (should probably fail on that?)
```json
{
  "Yield": {
    "span": "0:5",
    "argument": null,
    "delegate": false
  }
}
```
