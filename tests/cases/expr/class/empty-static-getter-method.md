### Source
```js parse:expr
class {
    static get getter() {}
}
```

### Output: minified
```js
class{static get getter(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:36",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "19:34",
          "name": {
            "Ident": {
              "span": "23:29",
              "name": "getter"
            }
          },
          "kind": "Get",
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
          "asynchronous": false,
          "is_static": true
        }
      }
    ]
  }
}
```
