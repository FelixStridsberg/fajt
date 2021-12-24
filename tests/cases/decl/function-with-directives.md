```js
function fn() {
    "use strict";
    'custom1';
    "custom2";
}
```

```js min
function fn(){"use strict";'custom1';"custom2";}
```

```json
{
  "FunctionDecl": {
    "span": "0:65",
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
      "span": "14:65",
      "directives": [
        {
          "value": "use strict",
          "delimiter": "\""
        },
        {
          "value": "custom1",
          "delimiter": "'"
        },
        {
          "value": "custom2",
          "delimiter": "\""
        }
      ],
      "statements": []
    }
  }
}
```
