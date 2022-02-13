### Source
```js
class Test extends Test2 {
    method() {
        async function* fn() {
            super.a;
        }
    }
}
```

### Output: error
