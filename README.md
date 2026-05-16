# fabrik-language-test

A polyglot fixture repo. Every top-level directory is a minimal, real,
independently testable project in a different stack. Fabrik points a
worker at this repo and we file one ticket per stack. The worker has
to clone, detect the stack, write a test + the code, run the
quality-gate sidecar against the right toolchain, and open a green PR.

Closes the live-test gate on fabrik#199 (language-agnostic worker).

## Stacks

| Dir | Language | Toolchain | Verify command |
|---|---|---|---|
| `go/`     | Go            | `go.mod`                     | `go test ./...` |
| `r/`      | R + testthat  | `DESCRIPTION` + `testthat/`  | `Rscript -e 'testthat::test_dir("tests/testthat")'` |
| `rust/`   | Rust          | `Cargo.toml`                 | `cargo test` |
| `dotnet/` | .NET (C#)     | `*.csproj` + xUnit           | `dotnet test` |
| `ruby/`   | Ruby + RSpec  | `Gemfile`                    | `bundle exec rspec` |

Each stack ships with one trivial passing function + one passing test.
Tickets ask the worker to add one more function + one more test in the
same dir.

## How tickets land here

The repo is wired up on a fabrik tenant. File an issue on this repo
with the standard concrete-ticket shape (`## Implementation` numbered,
`## Acceptance` checkboxes, named files, runnable verification, named
test) and the worker picks it up on its next heartbeat. The worker
opens a PR; CI in this repo runs the per-stack verify command. Green
PR + a `ticket.intake_classified` event with `route: direct_implement`
is the closing evidence for fabrik#199.

## Why a polyglot repo, not five repos

One tenant, one forge config, one CI surface. The QG sidecar autodetects
the stack from the touched paths, so each ticket exercises only its
language. Tradeoff: a polyglot monorepo is not how real customer repos
look, so if the worker has a clone-and-detect issue *with the mixture*
itself, this fixture wouldn't expose it. We accept that — the gate is
about the language coverage, not about packaging realism.
