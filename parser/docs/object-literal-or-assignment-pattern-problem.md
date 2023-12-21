# The object literal or assignment pattern ambiguity problem

All of these rows are valid expressions:
```js
[a, b, c]
[a, b, c] = {}
[a, {b = c}] = {}
{a, b, c}
{a, b, c} = {}
{a, b = c} = {}
```

But these are not valid expressions:
```js
[a, {b = c}]
{a, b = c}
```

I.e. a literal cannot contain an object with initializer. That is only as the
left side of an assignment.

The problem is that we do not know if the syntax is valid until after the
expression is parsed. This is covered by the `CoverInitializedName` in the
`ObjectLiteral` production, but only allowed in the `ObjectAssignmentPattern`.


# Potential solution

When the `CoverInitializedName` is covered inside an `ObjectLiteral`, we can
assume that we are really in an `ObjectAssignmentPattern`. However, just
converting the current production would not work since we may be inside a
nested literal. If we are inside a nested literal that whole nest must be
converted to an assignment pattern.

A potential solution for this may be similar to the [arrow function level
problem](./arrow-function-level-problem.md). We return error and handle that on
the literal parsing level and restart the parsing as assignment pattern
instead.

This may be a bit harder since the literals are nested, so we must make sure
the level we see the error are the top level of the current nest of literals.


## How to know when to stop bubble

When we hit an `CoverInitializedName` in an object literal, we must let that
error bubble up through all nested object and nested object literal.

We can do this in a few different ways, but all ways requires that we know
when we are no longer in a nested literal context.

For example:
```js
[{b = c} = {}]
```

This should be parsed as:
```
ArrayLiteral:
    Assignment:
        ObjectAssignmentPattern:
            ...
```

I.e. when we find the `CoverInitializedName` production in the object literal,
we should only "bubble" up to the parenthesized expression and re-parse as
assignment pattern, not the whole way up to the array literal.

However, this expression:
```js
[{b = c}] = {}
```

Should be parsed as:
```
Assignment:
    ArrayAssigmentPattern:
        ObjectAssignmentPattern:
            ...
```

I.e. it should bubble passed the array literal and convert that to assignment
pattern too.

The common pattern seems to be that we should bubble up to the left hand side
of the assignment expression and restart from there.
