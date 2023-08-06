### Source
```js parse:expr
{ a = 1, b: c().d } = e
```

### Output: minified
```js
{a=1,b:c().d}=e
```

### Output: ast

