# mail-news-letter

- Email news-letter in rust
- "ZERO TO PRODUCTION IN RUST" book study project.

---
## 62

## Project Setup:

- As the default linker is a little slow use the lld linker:
  ````toml
  # .cargo/config.toml
  # On Windows
  # ```
  # cargo install -f cargo-binutils
  # rustup component add llvm-tools-preview
  # ```
  [target.x86_64-pc-windows-msvc]
  rustflags = ["-C", "link-arg=-fuse-ld=lld"]
  [target.x86_64-pc-windows-gnu]
  rustflags = ["-C", "link-arg=-fuse-ld=lld"]
  # On Linux:
  # - Ubuntu, `sudo apt-get install lld clang`
  # - Arch, `sudo pacman -S lld clang`
  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
  # On MacOS, `brew install michaeleisel/zld/zld`
  [target.x86_64-apple-darwin]
  rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
  [target.aarch64-apple-darwin]
  rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
  ````
- cargo-watch

  ```console
  cargo install cargo-watch
  cargo watch -x check -x test -x run
  ```

- cargo test
- Code Coverage:
  Use cargo tarpaulin

  ```console
  cargo install cargo-tarpaulin
  cargo tarpaulin --ignore-tests
  ```

- Linting:

  ```console
  rustup component add clippy
  cargo clippy
  cargo clippy -- -D warnings
  ```

- Formatting:

  ```console
  rustup component add rustfmt
  cargo fmt
  cargo fmt -- --check
  ```

- Security Vulnerabilities:

  ```console
  cargo install cargo-audit
  cargo audit
  ```

- Use ready to go CI Pipelines:
  - [GitHub Actions](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3)
  - [CircleCI](https://gist.github.com/LukeMathWalker/6153b07c4528ca1db416f24b09038fca)
  - [GitLab CI](https://gist.github.com/LukeMathWalker/d98fa8d0fc5394b347adf734ef0e85ec)
  - [Travis](https://gist.github.com/LukeMathWalker/41c57a57a61c75cc8a9d137a8d41ec10)

## Capturing Requirements: User Stories

- The format of user stories is:
  > As a ...,
  > I want to ...,
  > So that ...
- User stories are:

  - As a blog visitor,
    I want to subscribe to the newsletter,
    So that I can receive email updates when new content is published on the blog;
  - As the blog author,
    I want to send an email to all my subscribers,
    So that I can notify them when new content is published;
  - As a subscriber,
    I want to be able to unsubscribe from the newsletter,
    So that I can stop receiving email updates from the blog.

- We will not add features to
  - manage multiple newsletters;
  - segment subscribers in multiple audiences;
  - track opening and click rates.

### User Story 1:

> As a blog visitor,
> I want to subscribe to the newsletter,
> So that I can receive email updates when new content is published on the blog;

- We will work in iterations: each iterations takes a fixed amount of time and gives us a slightly better version of the product, improving the experience of our users.
- We are iterating on product features, not engineerng quality: the code produced in each iteration will be tested and properly documented even if it only delivers a tiny, fully functional feature.

#### Choose Web Framework:

- [Choosing a Rust web framework, 2020 edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
- Using actix-web:
  - [website](https://actix.rs/)
  - [documentation](https://docs.rs/actix-web/4.0.1/actix_web/index.html)
  - [examples](https://github.com/actix/examples)

### Table driven test | Parameterized test

### Database: 
* PostgreSQL: postgres | postgres

---

## References:

- https://trstringer.com/postgresql-neovim
- https://github.com/guysherman/pg.nvim
