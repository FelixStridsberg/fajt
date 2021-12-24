```js
class cls {
    async method1() {}
}
```

```js min
class cls{async method1(){}}
```

```json
{
  "Class": {
    "span": "0:36",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:34",
          "name": {
            "Ident": {
              "span": "22:29",
              "name": "method1"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "29:31",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "32:34",
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
