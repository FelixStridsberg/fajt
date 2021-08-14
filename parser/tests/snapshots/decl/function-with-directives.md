```js
function fn() {
  "use strict"
  'custom1';
  "custom2"
}
```

```json
{
  "FunctionDecl": {
    "span": "0:57",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:13",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "14:57",
      "directives": [
        "use strict",
        "custom1",
        "custom2"
      ],
      "statements": []
    }
  }
}
```
