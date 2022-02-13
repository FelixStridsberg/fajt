### Source
```js
class Test extends Test2 {
    method() {
        function* fn(a = super.a) {}
    }
}
```

### Output: error
