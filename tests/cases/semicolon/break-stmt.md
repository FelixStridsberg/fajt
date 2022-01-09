### Source
```js check-format:no
break
break;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:12",
    "body": [
      {
        "Break": {
          "span": "0:5",
          "label": null
        }
      },
      {
        "Break": {
          "span": "6:12",
          "label": null
        }
      }
    ]
  }
}
```
