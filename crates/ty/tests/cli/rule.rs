use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;

fn ty_cmd() -> Command {
    let mut cmd = Command::new(get_cargo_bin("ty"));
    cmd.env_clear();
    cmd
}

#[test]
fn rule_default_output() {
    assert_cmd_snapshot!(ty_cmd().args(["explain", "rule", "invalid-return-statement"]), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    # invalid-return-statement

    Default level: error | Stable (since 0.0.100)

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

    ----- stderr -----
    "#);
}

#[test]
fn rule_json_output() {
    assert_cmd_snapshot!(ty_cmd().args(["explain", "rule", "invalid-return-statement", "--output-format", "json"]), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "name": "invalid-return-statement",
      "summary": "detects incompatible return values and generator return annotations",
      "documentation": "## What it does\n\nChecks for `return` statements where the value returned is inconsistent with the function's\nreturn-type annotation. It also checks whether a generator function's return annotation allows the\ngenerator object it produces.\n\nA bare `return` is equivalent to `return None`. If execution can reach the end of the function, ty\nchecks the implicit return separately with `missing-return-statement` or, for a stub-like body,\n`missing-return-in-stub-body`.\n\nIn a synchronous generator annotated with `Generator[YieldType, SendType, ReturnType]`, an explicit\n`return` must produce a value assignable to `ReturnType`.\n\nty accepts the old name `invalid-return-type` and warns about the rename in configuration and inline\nignores. If you used `# ty: ignore[invalid-return-type]` for a nonempty function that falls through,\nuse `# ty: ignore[missing-return-statement]` to keep suppressing that diagnostic.\n\n## Why is this bad?\n\nA caller relying on the annotated return type can receive a value of an unexpected type and fail at\nruntime.\n\n## Examples\n\n```python\nfrom collections.abc import Generator\n\n\ndef func(coinflip: bool) -> int:\n    if coinflip:\n        return \"a\"  # error: [invalid-return-statement]\n    else:\n        return  # error: [invalid-return-statement]\n\n\ndef gen_func() -> Generator[int, None, str]:\n    yield 42\n    return b\"oops, a bytestring\"  # error: [invalid-return-statement]\n```\n\nBoth functions below yield integers. Annotate `numbers` with `collections.abc.Generator[int]` and\n`async_numbers` with `collections.abc.AsyncGenerator[int]`.\n\n```python\nfrom collections.abc import Generator\n\n\n# error: [invalid-return-statement]\ndef numbers() -> int:\n    yield 1\n\n\n# error: [invalid-return-statement]\nasync def async_numbers() -> Generator[int]:\n    yield 1\n```",
      "default_level": "error",
      "status": {
        "type": "stable",
        "since": "0.0.100"
      }
    }
    ----- stderr -----
    "###);
}

#[test]
fn preview_rule_status() -> anyhow::Result<()> {
    let output = ty_cmd()
        .args(["explain", "rule", "missing-direct-dependency"])
        .output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert!(
        stdout.contains("Default level: ignore | Preview (since 0.0.76)"),
        "{stdout}"
    );

    let output = ty_cmd()
        .args([
            "explain",
            "rule",
            "missing-direct-dependency",
            "--output-format",
            "json",
        ])
        .output()?;
    assert!(output.status.success());
    let rule: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        rule["status"],
        serde_json::json!({"type": "preview", "since": "0.0.76"})
    );

    Ok(())
}

#[test]
fn rule_unknown() {
    assert_cmd_snapshot!(ty_cmd().args(["explain", "rule", "does-not-exist"]), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    ty failed
      Cause: Unknown rule `does-not-exist`
    ");
}

/// Without a selector, we default to explaining all rules, so just assert that the command succeeds
/// rather than snapshotting the documentation for every rule.
#[test]
fn rule_no_selector() -> anyhow::Result<()> {
    assert!(ty_cmd().args(["explain", "rule"]).status()?.success());

    Ok(())
}
