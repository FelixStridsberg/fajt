### Source
```js parse:expr
class MyClass {}
```

### Output: minified
```js
class MyClass{}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:16",
    "identifier": {
      "span": "6:13",
      "name": "MyClass"
    },
    "super_class": null,
    "body": []
  }
}
```
