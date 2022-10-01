### Source
Regexp may contain code points that are not recognized by the parser.
This catches that edge case.

```js check-format:no
/#/;
/@/;
/☂/;
```

### Output: ast
