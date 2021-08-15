```js
class cls { set setter(a) {} }
```

```json
{
  "Class": {
    "span": "0:30",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:28",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "setter"
            }
          },
          "kind": "Set",
          "parameters": {
            "span": "22:25",
            "bindings": [
              {
                "span": "23:24",
                "pattern": {
                  "Ident": {
                    "span": "23:24",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "26:28",
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
