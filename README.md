# `controlflow_inspect`

![Crates.io License](https://img.shields.io/crates/l/controlflow_inspect)
[![Crates.io Version](https://img.shields.io/crates/v/controlflow_inspect)](https://crates.io/crates/controlflow_inspect/)
![GitHub branch check runs](https://img.shields.io/github/check-runs/komar007/controlflow_inspect/main)
[![docs.rs](https://img.shields.io/docsrs/controlflow_inspect)](https://docs.rs/controlflow_inspect)
![Static Badge](https://img.shields.io/badge/msrv-1.55-blue)

`inspect_break` and `inspect_continue` for `ControlFlow`.

## Overview

Analogically to `inspect` and `inspect_err` for inspecting values inside `Result`, this crate
introduces `inspect_break` and `inspect_continue` for `ControlFlow`. Neither the break nor the
continue variant of `ControlFlow` is more important than the other, so the methods are named
symmetrically.
