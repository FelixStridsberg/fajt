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

It turns out that it is non trivial to know when to stop bubble.

Consider this scenario:
```js
[{a =
//  ^
// This is not a valid token in an object literal, it is either invalid syntax
// or an assignment pattern.
```

In this scenario we should rewind the parser and re-parse as an assignment pattern.

But should we rewind to the `{` or the `[`? That depends on which one is followed
by a `=`. And that we do now know at this point.

We could be in this scenario:
```js
[{a = 1}] = b
// Assignment
//   ArrayAssignmentPattern
//     ObjectAssignmentPattern
```

```js
[{a = 1} = b]
// ArrayLiteral
//   Assignment
//     ObjectAssignmentPattern
```

The simplest solution may be to rewind to the start of the current literal, and
then later rewind again if a literal value turns out to be an assignment
pattern.

(Note that there are several levels of productions not represented in the
following examples)

From the first example above:
```js
   [{a =
//     ^
// Invalid object literal, may be an assignment pattern
   [{
//  ^
// Re-parse as assignment pattern

   [{a = 1}]
//         ^
// An array literal contained assignment pattern as value

   [
// ^
// Re-parse as assignment pattern

   [{a = 1}] = b
// Successfully parsed as:
// Assignment
//   ArrayAssignmentPattern
//     ObjectAssignmentPattern
```

From the second example above:
```js
   [{a =
//     ^
// Invalid object literal, may be an assignment pattern

   [{
//  ^
// Re-parse as assignment pattern

   [{a = 1} = b]
// Successfully parsed as:
// ArrayLiteral
//   Assignment
//     ObjectAssignmentPattern
```


## Performance drawbacks

The obvious drawbacks with this approach is that for large nested objects, we
may need to re-read a lot of data several times.

But the main goal of this parser is simplicity, so that will be acceptable at
this time.
