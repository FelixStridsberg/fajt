```js
class { *method1() {} }
```

```json
{
  "Class": {
    "span": "0:23",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "8:21",
          "name": {
            "Ident": {
              "span": "9:16",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "16:18",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "19:21",
            "directives": [],
            "statements": []
          },
          "generator": true,
          "asynchronous": false
        }
      }
    ]
  }
}
```
