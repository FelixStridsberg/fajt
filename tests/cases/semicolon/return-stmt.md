### Source
```js check-format:no
return
return;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:14",
    "directives": [],
    "body": [
      {
        "Return": {
          "span": "0:6",
          "argument": null
        }
      },
      {
        "Return": {
          "span": "7:14",
          "argument": null
        }
      }
    ]
  }
}
```
