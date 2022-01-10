### Source
```js parse:expr
(a, b, ...rest) => {
    ;
}
```

### Output: minified
```js
(a,b,...rest)=>{}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:28",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:15",
      "bindings": [
        {
          "span": "1:2",
          "pattern": {
            "Ident": {
              "span": "1:2",
              "name": "a"
            }
          },
          "initializer": null
        },
        {
          "span": "4:5",
          "pattern": {
            "Ident": {
              "span": "4:5",
              "name": "b"
            }
          },
          "initializer": null
        }
      ],
      "rest": {
        "Ident": {
          "span": "10:14",
          "name": "rest"
        }
      }
    },
    "body": {
      "Body": {
        "span": "19:28",
        "directives": [],
        "statements": [
          {
            "Empty": {
              "span": "25:26"
            }
          }
        ]
      }
    }
  }
}
```