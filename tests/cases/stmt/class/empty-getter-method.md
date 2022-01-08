### Input
```js
class cls {
    get getter() {}
}
```

```js min
class cls{get getter(){}}
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
          "span": "16:31",
          "name": {
            "Ident": {
              "span": "20:26",
              "name": "getter"
            }
          },
          "kind": "Get",
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
          "generator": false,
          "asynchronous": false
        }
      }
    ]
  }
}
```
