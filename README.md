# space

## Usage

`space [program]`

### New

`space new`

starts wizard to create and change to new space.  Determined by spaces in $HOME/.spaces.yml file: e.g.

```
repos:
  - my-awesome-repo-1
  - my-awesome-repo-2
  - my-awesome-repo-3
```

Authentication happens through username and token stored as the following env variables:

```
export GIT_USERNAME="some-username"
export GIT_PASSWORD="some-token-or-password"
```

path to the directory will be stored in clipboard for easy navigation afterwards

### Purge

`space purge`

same as rm -rf $HOME/spaces
