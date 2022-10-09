### Source
This is a potentially tricky case when reading CoverCallExpressionAndAsyncArrow head production.

```js check-format:no
var a = async("(", '(', `(`, /(/ /*(*/);
```

### Output: minified
### Output: ast
