```js
class cls { async *method1() {} }
```

```json
{
  "Class": {
    "span": "0:33",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:31",
          "name": {
            "Ident": {
              "span": "19:26",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "26:28",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "29:31",
            "directives": [],
            "statements": []
          },
          "generator": true,
          "asynchronous": true
        }
      }
    ]
  }
}
```
