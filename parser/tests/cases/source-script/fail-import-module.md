### Input
```js source:script
export 'module'
```

### Output: ast
```json
{
  "SyntaxError": [
    "'export' cannot appear in a 'script' source.",
    "0:6"
  ]
}
```
