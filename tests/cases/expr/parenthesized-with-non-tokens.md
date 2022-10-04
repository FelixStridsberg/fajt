### Source
This is an edge case where the cover production for parenthesized expression parses
code points that is not a token.

```js parse:expr
((a = /#/), (b = /@/), (c = /☂/));
```

### Output: ast
