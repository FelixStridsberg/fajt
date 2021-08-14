```js
class { async *method1() {} }
```

```json
{
  "Class": {
    "span": "0:29",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "8:27",
          "name": {
            "Ident": {
              "span": "15:22",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:24",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "25:27",
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
