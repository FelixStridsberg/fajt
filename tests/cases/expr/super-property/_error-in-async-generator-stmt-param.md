### Source
```js
class Test extends Test2 {
    method() {
        async function* fn(a = super.a) {}
    }
}
```

### Output: error
