### Source
```js check-format:no
continue
continue;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:18",
    "directives": [],
    "body": [
      {
        "Continue": {
          "span": "0:8",
          "label": null
        }
      },
      {
        "Continue": {
          "span": "9:18",
          "label": null
        }
      }
    ]
  }
}
```
