```js
try {} catch (e) {} finally {}
```

```js min
try{}catch(e){}finally{}
```

```json
{
  "Try": {
    "span": "0:30",
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
    "finalizer": {
      "span": "28:30",
      "statements": []
    }
  }
}
```
