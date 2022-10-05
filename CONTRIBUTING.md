# Contributing to l1t

First, I'd like to thank you for taking the time to
contribute to `l1t`. I'm not perfect and make mistakes
and I appreciate any that I can get.

## How to Contribute

You can follow these guidelines to contribute.

### Running l1t for Development

To contribute to `l1t`, you can start by forking an
instance of the repo.

- Fork this repository

- Clone your fork

```bash
git clone https://github.com/<your_fork>/l1t
cd ./l1t
```
- Build the project for development:

```bash
make dev
```

- Run and test the development instance:

```bash
./build/l1t.dev
```

The developmenet instance as all warnings turned on. PR's
that compile with warnings will be asked to be modified
so that no warnings/errors are generated. If, for some reason,
the warnings ***must*** be present, the you most likely need
to attempt the feature or fix you are trying to make a
different way.

### Submitting a PR

Once you have made your changes and everything is good,
open a PR and select the fork you have modified to merge
into `main`. Make sure to include the required information
from the PR template.
