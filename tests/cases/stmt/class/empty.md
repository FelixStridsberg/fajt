### Source
```js parse:stmt
class cls {}
```

### Output: minified
```js
class cls{}
```

### Output: ast
```json
{
  "ClassDecl": {
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
