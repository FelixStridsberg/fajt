### Source
```js parse:expr
class {
    get() {}
    set() {}
}
```

### Output: minified
```js
class{get(){}set(){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:35",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:20",
          "name": {
            "Ident": {
              "span": "12:15",
              "name": "get"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "15:17",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "18:20",
            "directives": [],
            "statements": []
          },
          "generator": false,
          "asynchronous": false
        }
      },
      {
        "Method": {
          "span": "25:33",
          "name": {
            "Ident": {
              "span": "25:28",
              "name": "set"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "28:30",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "31:33",
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
