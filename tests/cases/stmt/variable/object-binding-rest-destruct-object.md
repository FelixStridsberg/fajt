### Source
```js parse:stmt
var { ...{ a } } = c;
```

### Output: minified
```js
var{...{a}}=c;
```

### Output: ast

