### Source
This edge cases where the cover production for parenthesized expression tries to
read until next matching parenthesize.

```js
(/(/ /*(*/, "(", '(', `(`);
(/)/ /*)*/, ")", ')', `)`);
```

### Output: ast
