### Input
```js
class MyClass extends SuperClass {}
```

```js min
class MyClass extends SuperClass{}
```

```json
{
  "Class": {
    "span": "0:35",
    "identifier": {
      "span": "6:13",
      "name": "MyClass"
    },
    "super_class": {
      "IdentRef": {
        "span": "22:32",
        "name": "SuperClass"
      }
    },
    "body": []
  }
}
```
