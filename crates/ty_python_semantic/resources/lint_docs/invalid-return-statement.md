## What it does

Checks for `return` statements where the value returned is inconsistent with the function's
return-type annotation. It also checks whether a generator function's return annotation allows the
generator object it produces.

A bare `return` is equivalent to `return None`. If execution can reach the end of the function, ty
checks the implicit return separately with `missing-return-statement` or, for a stub-like body,
`missing-return-in-stub-body`.

In a synchronous generator annotated with `Generator[YieldType, SendType, ReturnType]`, an explicit
`return` must produce a value assignable to `ReturnType`.

ty accepts the old name `invalid-return-type` and warns about the rename in configuration and inline
ignores. If you used `# ty: ignore[invalid-return-type]` for a nonempty function that falls through,
use `# ty: ignore[missing-return-statement]` to keep suppressing that diagnostic.

## Why is this bad?

A caller relying on the annotated return type can receive a value of an unexpected type and fail at
runtime.

## Examples

```python
from collections.abc import Generator


def func(coinflip: bool) -> int:
    if coinflip:
        return "a"  # error: [invalid-return-statement]
    else:
        return  # error: [invalid-return-statement]


def gen_func() -> Generator[int, None, str]:
    yield 42
    return b"oops, a bytestring"  # error: [invalid-return-statement]
```

Both functions below yield integers. Annotate `numbers` with `collections.abc.Generator[int]` and
`async_numbers` with `collections.abc.AsyncGenerator[int]`.

```python
from collections.abc import Generator


# error: [invalid-return-statement]
def numbers() -> int:
    yield 1


# error: [invalid-return-statement]
async def async_numbers() -> Generator[int]:
    yield 1
```
