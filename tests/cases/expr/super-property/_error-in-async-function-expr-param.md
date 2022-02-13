### Source
```js
class Test extends Test2 {
    method() {
        const a = async function (b = super.a) {};
    }
}
```

### Output: error
