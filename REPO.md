# Repositories

Repositories allow you to how `l1t` levels on web server that anyone can access.

## Using Repositories

To add a repo to your settings, add a line in your `$HOME/.l1t/repositories.l1t_conf` file:

```
My Repo Name = http://myrepourl.com
```

Where the left side of the `=` is the name of your repo and the right is the URL.

## Hosting Repositories

To host a repository, you will need a web server that serves a `/l1t` route which returns the following JSON response:

```json
{
    "levels": [
        {
            "source": "<relative_path_to_level>",
            "name": "Level Name",
            "author": "Level Author",
            "description": "Leve Description"
        },
        ...
    ]
}
```

This response acts as the listing for every level that the repo hosts. All levels in this listing are expected to valid
relative paths to the level file and contain the `name`, `author`, and `description` for each level. Any extra values returned
will be ignored by the client. Each level must be a subroute of the `/l1t` route.

For example, if your main repo path is `http://myrepo.com/` and you host one level (`Level 1`) hosted at `http://myrepo.com/l1t/level1.l1t`,
then a get request to the path `http://myrepo.com/l1t` should return:

```json
{
    "levels": [
        {
            "source": "level1.l1t",
            "name": "Level 1",
            "author": "you",
            "description": "A Description"
        }
    ]
}
```

If `Level 1` was instead hosted at `http://myrepo.com/l1t/levels/level1.l1t`, then `source` should be replaced with `levels/level1.l1t`.
Note that since each level must have a source as a relative route of the main route, no level in this scenario can be hosted outside
of the `/l1t` route directory.

An example repository is provided in `examples/repo` and can be started with `python`:

```bash
git clone https://github.com/alex-laycalvert/l1t
cd l1t/examples/repo
python -m http.server
```

Once it's started, add the following line in your `$HOME/.l1t/repositories.l1t_conf` file:

```
My Local Repo = http://localhost:8000/
```

It can be tested by starting `l1t` and selecting `ONLINE` from the menu options.
