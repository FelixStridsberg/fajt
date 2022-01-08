### Input
```js parse:expr
async (a, b, ...rest) => { ; }
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:30",
    "asynchronous": true,
    "binding_parameter": false,
    "parameters": {
      "span": "6:21",
      "bindings": [
        {
          "span": "7:8",
          "pattern": {
            "Ident": {
              "span": "7:8",
              "name": "a"
            }
          },
          "initializer": null
        },
        {
          "span": "10:11",
          "pattern": {
            "Ident": {
              "span": "10:11",
              "name": "b"
            }
          },
          "initializer": null
        }
      ],
      "rest": {
        "Ident": {
          "span": "16:20",
          "name": "rest"
        }
      }
    },
    "body": {
      "Body": {
        "span": "25:30",
        "directives": [],
        "statements": [
          {
            "Empty": {
              "span": "27:28"
            }
          }
        ]
      }
    }
  }
}
```
