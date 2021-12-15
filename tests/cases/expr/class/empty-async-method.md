```js
class { async method1() {} }
```

```json
{
  "Class": {
    "span": "0:28",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "8:26",
          "name": {
            "Ident": {
              "span": "14:21",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "21:23",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "24:26",
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
