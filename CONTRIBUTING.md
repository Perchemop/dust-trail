# CONTRIBUTING.md

Contributions to Dust Trail are welcome. All work lands on `main` via a
reviewed pull request with CI green (guide §5).

1. Fork the repo, create a branch, make a small focused slice.
2. Add or update tests for any behaviour change.
3. Run `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings`
   locally if you can; CI enforces them regardless.
4. Open a PR using `.github/PULL_REQUEST_TEMPLATE.md`.
5. No proprietary source, assets, or trade dress (clean-room rule, guide
   Part I §2.2).

By submitting a PR, you license your contribution under the project's dual
MIT OR Apache-2.0 licence.
