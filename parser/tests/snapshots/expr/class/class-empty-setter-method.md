```js
class { set setter(a) {} }
```

```json
{
  "Class": {
    "span": "0:26",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "8:24",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "setter"
            }
          },
          "kind": "Set",
          "parameters": {
            "span": "18:21",
            "bindings": [
              {
                "span": "19:20",
                "pattern": {
                  "Ident": {
                    "span": "19:20",
                    "name": "a"
                  }
                },
                "initializer": null
              }
            ],
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
