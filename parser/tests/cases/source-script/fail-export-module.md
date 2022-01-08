### Input
```js source:script
import 'module'
```

### Output: ast
```json
{
  "SyntaxError": [
    "'import' cannot appear in a 'script' source.",
    "0:6"
  ]
}
```
