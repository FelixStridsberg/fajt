### Source
```js parse:stmt check-format:no
class cls { ; }
```

### Output: minified
```js
class cls{}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:15",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": []
  }
}
```
