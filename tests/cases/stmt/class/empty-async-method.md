```js
class cls { async method1() {} }
```

```json
{
  "Class": {
    "span": "0:32",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:30",
          "name": {
            "Ident": {
              "span": "18:25",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "25:27",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "28:30",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": true
        }
      }
    ]
  }
}
```
