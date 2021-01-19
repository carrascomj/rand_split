# :alien: As an user
If you would like to have a feature implemented or you noticed a bug, do not hesitate
to write an [issue in the repository](https://github.com/carrascomj/chordclust/issues).

# :computer: As a developer
## :art: Style
* Use [semantic commits](https://seesparkbox.com/foundry/semantic_commit_messages).
* 23 incremental Pull Requests are better than 1 commit of 27,000 lines.
* Before commiting, [format the code](https://github.com/rust-lang/rustfmt)
and search for [lint warnings](https://github.com/rust-lang/rust-clippy) (both
must be checked to pass CI):
  ```shell
  cargo fmt
  cargo clippy
  ```
* Tests, benchmarks and documentation are always welcome! The readme is automatically generated from [`src/lib.rs`](https://github.com/carrascomj/chordclust/blob/trunk/src/lib.rs), so make your changes there or at [README.tpl](https://github.com/carrascomj/chordclust/blob/trunk/README.tpl).

## :rocket: Enhancements, bugfixes or feature
The API is not set in stone, changes and propositions are considered and welcomed.

1. Look up similar [issues](https://github.com/carrascomj/chordclust/issues).
2. [Write an issue](https://github.com/carrascomj/chordclust/issues/new).
3. [Fork](https://docs.github.com/en/enterprise/2.13/user/articles/fork-a-repo) the repository.
  ```shell
  # https
  git clone https://github.com/carrascomj/chordclust.git
  # or ssh
  git clone git@github.com:carrascomj/chordclust.git
  ```
4. Branch from trunk.
  ```shell
  git checkout -b 'feat-incrediblefeature'
  ```
5. Commit a whole bunch of stuff ([this video](https://www.youtube.com/watch?v=BaPexytJFTI)
  might be helpful to understand [Git](https://git-scm.com/)).
6. Submit a [Pull Request](https://github.com/carrascomj/chordclust/pulls) with your feature/bug fix.
7. Get the Pull Request approved (CI must pass).  

The steps of writing the issue before submitting anything are so we all are
respectful with each other's time. Maybe another developer can put you in the
right direction before trying to write any code!
