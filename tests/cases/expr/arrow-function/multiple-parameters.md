### Input
```js
(a, b, ...rest) => { ; }
```

```json
{
  "ArrowFunction": {
    "span": "0:24",
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
        "span": "19:24",
        "directives": [],
        "statements": [
          {
            "Empty": {
              "span": "21:22"
            }
          }
        ]
      }
    }
  }
}
```
