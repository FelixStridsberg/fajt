### Source
```js parse:expr
class {
    get getter() {}
}
```

### Output: minified
```js
class{get getter(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:29",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:27",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "getter"
            }
          },
          "kind": "Get",
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
          "generator": false,
          "asynchronous": false,
          "is_static": false
        }
      }
    ]
  }
}
```
