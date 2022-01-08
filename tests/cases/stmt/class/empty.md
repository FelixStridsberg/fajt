### Input
```js
class cls {}
```

### Output: minified
```js min
class cls{}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:12",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": []
  }
}
```
