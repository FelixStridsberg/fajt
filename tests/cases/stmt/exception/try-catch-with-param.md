### Source
```js parse:stmt
try {} catch (e) {}
```

### Output: minified
```js
try{}catch(e){}
```

### Output: ast
```json
{
  "Try": {
    "span": "0:19",
    "block": {
      "span": "4:6",
      "statements": []
    },
    "handler": {
      "span": "7:19",
      "parameter": {
        "Ident": {
          "span": "14:15",
          "name": "e"
        }
      },
      "body": {
        "span": "17:19",
        "statements": []
      }
    },
    "finalizer": null
  }
}
```
