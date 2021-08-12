```js
class { method1() {} }
```

```json
{
  "Class": {
    "span": "0:22",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "8:20",
          "name": {
            "Ident": {
              "span": "8:15",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "15:17",
            "bindings": [],
            "rest": null
          },
          "body": [],
          "generator": false,
          "asynchronous": false
        }
      }
    ]
  }
}
```
