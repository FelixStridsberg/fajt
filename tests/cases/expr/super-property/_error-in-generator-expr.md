### Source
```js
class Test extends Test2 {
    method() {
        const a = function* () {
            super.a;
        };
    }
}
```

### Output: error
