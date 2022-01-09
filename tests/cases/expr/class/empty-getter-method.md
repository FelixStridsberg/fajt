### Source
```js parse:expr
class { get getter() {} }
```

### Output: ast
```json
{
  "Class": {
    "span": "0:25",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "8:23",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "getter"
            }
          },
          "kind": "Get",
          "parameters": {
            "span": "18:20",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "21:23",
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
