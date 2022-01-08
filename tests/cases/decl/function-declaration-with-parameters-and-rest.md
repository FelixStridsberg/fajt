### Input
```js
function fn(a, ...b) {}
```

### Output: minified
```js min
function fn(a,...b){}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:23",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:20",
      "bindings": [
        {
          "span": "12:13",
          "pattern": {
            "Ident": {
              "span": "12:13",
              "name": "a"
            }
          },
          "initializer": null
        }
      ],
      "rest": {
        "Ident": {
          "span": "18:19",
          "name": "b"
        }
      }
    },
    "body": {
      "span": "21:23",
      "directives": [],
      "statements": []
    }
  }
}
```
