### Source
```js parse:expr
import.non_existent
```

### Output: ast
```json
{
  "UnexpectedIdent": {
    "span": "7:19",
    "name": "non_existent"
  }
}
```
