### Source
```js parse:stmt
class cls {
    method() {
        ({ a: this.a } = { a: 1 });
    }
}
```

### Output: minified
### Output: ast
