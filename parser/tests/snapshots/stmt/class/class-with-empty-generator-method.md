```js
class cls { *method1() {} }
```

```json
{
  "Class": {
    "span": "0:27",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:25",
          "name": {
            "Ident": {
              "span": "13:20",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "20:22",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "23:25",
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
