# What is this, User-wise?
This is effectively an API for a Skeleton Repo.
Instead of manually filling in the details of the skeleton repo in order to flesh it out and make it usable, you can just call this app answer its prompts and it will create a fleshed out repo for you, initialize git, place a pre-commit hook (appropriate to the CI/CD code in-repo), and do an initial commit.

So you can just type a few words and have a fully functioning set-up.  Complete with local CI/CD and with scripts to almost instantly hook in remote CI/CD.  (Just upload the repo and the rest should be done for you; assuming reasonable things about your remote set-up.)

# What is this, Internals-wise?
Three things.
1. A terminal application that promps and takes inputs.
2. A templating engine that recursively walks through and fills in sections of a skeleton repo rolled up in its binary and writes it to disk.
3. A shell interface, that spawns sub-shells and runs simple commands to finish setting up. (e.g. initializing git, and thereby using your default git settings, and making the git pre-commit hook that it wrote executable)

# Does it work?
Yep. 100%

# What does it work for?
It is currently set-up with one specific skeleton repo baked in.
That repo is a Python repo with a terminal app framework set-up, some example commands written, some example tests written, and lots of other niceness like set-up and compatable-ized Formatters, Linters, Type-Checkers, Code Coverage calculators, etc.
It also has GitHub Actions and CircleCI scripts.  the CircleCI scripts cover all the real CI/CD work and the GitHub Actions script re-runs the auto-documentation code and then publishes it to the GitHub Pages website for the repo.

# What else might be done in the future?
Please see (and feel encouraged to add to) the "[Issues]([url](https://github.com/ethanmsl/incarnate/issues))" which includes enhancement requests.  Additionally:

1. More automation could be done on the remote side.  e.g. setting up GitHub or talking to CircleCI to register the remote repo.  -- Those are both almost instantaneous tasks however and perhaps execsively specific.  So they're hard *maybe*s on the improvement list.
2. Writing macros so templated repos can be auto-integrated.  This app has a little bit of hard coding for its inception skeleton.  Which was the cause of its original design.  But By extracting the relevant variables, template signatures, and doing this at compile time ahead of other compile time macros it can be easily generalized.  That is an improvement that is likely, but is not an immediate need.

# How can I run it?

## The simplest
If you're using an ARM64 mac (e.g. the M1) then you can use [homebrew]([url](https://brew.sh/)):
```bash
brew tap ethanmsl/incarnate
brew install incarnate
```

## Alternatives
Otherwise you can compile the binary natively with Cargo. 
Or (for mac_x86 and mac_arm) download a one from the releases version on this page.

The following code, run locally to the binary (however you got it) should, on a mac, ensure that it is runnable as a terminal command.
```bash
ISHALLBERUN=incarnate
xattr -d com.apple.quarantine $ISHALLBERUN
chmod +x $ISHALLBERUN
sudo cp $ISHALLBERUN /usr/local/bin/incarnate
```
