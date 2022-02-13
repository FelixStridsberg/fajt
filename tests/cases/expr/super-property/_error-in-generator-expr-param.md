### Source
```js
class Test extends Test2 {
    method() {
        const a = function* (b = super.a) {};
    }
}
```

### Output: error
