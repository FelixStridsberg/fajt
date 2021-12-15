```js
class cls { method1() {} }
```

```json
{
  "Class": {
    "span": "0:26",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:24",
          "name": {
            "Ident": {
              "span": "12:19",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "19:21",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "22:24",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": false
        }
      }
    ]
  }
}
```
