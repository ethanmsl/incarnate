( **Dev Use Note**: this uses a git submodule for the template assets, be sure to initialize and update if working on repo)

<!--toc:start-->

- [What is this, User-wise?](#what-is-this-user-wise)
- [Who is this for?](#who-is-this-for)
  - [Alternative applications:](#alternative-applications)
- [What is this, Internals-wise?](#what-is-this-internals-wise)
- [Does it work?](#does-it-work)
- [What does it work for?](#what-does-it-work-for)
- [What else might be done in the future?](#what-else-might-be-done-in-the-future)
- [How can I run it?](#how-can-i-run-it)
  - [The simplest](#the-simplest)
  - [Alternatives](#alternatives)
  - [Cloning Note](#cloning-note)
  <!--toc:end-->

# What is this, User-wise?

This is effectively an API for a Skeleton Repo.
Instead of manually filling in the details of the skeleton repo in order to flesh it out and make it usable, you can just call this app answer its prompts and it will create a fleshed out repo for you, initialize git, place a pre-commit hook (appropriate to the CI/CD code in-repo), and do an initial commit.

So you can just type a few words and have a fully functioning set-up. Complete with local CI/CD and with scripts to almost instantly hook in remote CI/CD. (Just upload the repo and the rest should be done for you; assuming reasonable things about your remote set-up.)

# Who is this for?

This is mostly for my and direct collaborators' use.
Synchronizing coding idioms and application creation, providing educational materials that can be direclty segued into usable code, and a way of providing interactive documentation in the form of runnable code skeletons.

If you've stumbled on this it's probably not for you, as it is, by design specific to work involving myself and collaborators and has solid, but distinct choices made to its template. (e.g. use of Poetry, Polars, structlog, etc.) You can just point the [git-submodule](https://github.com/ethanmsl/incarnate/blob/master/.gitmodules) elsewhere and customize the template to your liking if you wanted though.

## Alternative applications:

If you're looking for flexible templating applications:

- for Rust: I've had good success with [cargo generate](https://github.com/cargo-generate/cargo-generate), recently (late 2023).
  - e.g. setting up this [egui-template](https://github.com/ethanmsl/cargo_generate_eframe_template) fairly simply from a hard-coded repo, which can generate [this repo with deployed WASM](https://github.com/ethanmsl/etest).
- for Python: I don't have personal experience, but [cookiecutter](https://github.com/cookiecutter/cookiecutter) looked the most promising to me when I last surveyed options (late 2023).

# What is this, Internals-wise?

Three things.

1. A terminal application that promps and takes inputs.
2. A templating engine that recursively walks through and fills in sections of a skeleton repo rolled up in its binary and writes it to disk.
3. A shell interface, that spawns sub-shells and runs simple commands to finish setting up. (e.g. initializing git, and thereby using your default git settings, and making the git pre-commit hook that it wrote executable)

Note: This application hard-rolls templates into it. Speed and reliablity are fun side-effects of this, but its primary purposes is to ensure effort-free activation and synchronization when working with a team. (Also, it was a very early rust project and so was built around a goal that could be supported by a small, naive, body of code.)

# Does it work?

Yep. 100%

(historical: this was the output of a hackday project, hence the line above. It makes me smile so I'm leaving it. lol )

# What does it work for?

It is currently set-up with one specific skeleton repo baked in.
That repo is a Python repo with a terminal app framework set-up, some example commands written, some example tests written, and lots of other niceness like set-up and compatable-ized Formatters, Linters, Type-Checkers, Code Coverage calculators, etc.
It also has GitHub Actions and CircleCI scripts. the CircleCI scripts cover all the real CI/CD work and the GitHub Actions script re-runs the auto-documentation code and then publishes it to the GitHub Pages website for the repo.

It now also has some jupyter notebooks and example Polars code (mostly for collaborative education purposes).

# What else might be done in the future?

Please see (and feel encouraged to add to) the "[Issues](<[url](https://github.com/ethanmsl/incarnate/issues)>)" which includes enhancement requests. Additionally:

1. More automation could be done on the remote side. e.g. setting up GitHue or talking to CircleCI to register the remote repo. -- Those are both almost instantaneous tasks however and perhaps execsively specific. So they're hard *maybe*s on the improvement list.
2. Writing macros so templated repos can be auto-integrated. This app has a little bit of hard coding for its inception skeleton. Which was the cause of its original design. But By extracting the relevant variables, template signatures, and doing this at compile time ahead of other compile time macros it can be easily generalized. That is an improvement that is likely, but is not an immediate need.

# How can I run it?

## The simplest

If you're using an ARM64 mac (e.g. the M1) then you can use [homebrew](<[url](https://brew.sh/)>):

```bash
brew tap ethanmsl/incarnate
brew install incarnate
```

## Alternatives

Otherwise you can compile the binary natively with Cargo.
Or, for mac_aarch64 (M1+ macs), download the binary from the release version in the GitHub repo.

The following code, run locally to the binary (however you got it) should, on a mac, ensure that it is runnable as a terminal command.

```bash
ISHALLBERUN=incarnate
xattr -d com.apple.quarantine $ISHALLBERUN
chmod +x $ISHALLBERUN
sudo cp $ISHALLBERUN /usr/local/bin/incarnate
```

## Cloning Note

This repo uses [git-submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules) to store templates. The short version: after cloning run `git submodule init` and `git submodule update`. And run the latter command to update with the submodule repo(s).

Update: if you've installed [just](https://github.com/casey/just?tab=readme-ov-file#packages) you can simply run `just git-submodules` and it will take care of this.
