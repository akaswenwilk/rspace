# space

## Usage

`space [program]`

### New

`space new`

starts wizard to create and change to new space.  Determined by spaces in $HOME/.spaces.yml file: e.g.

```
username:
    my-token:
        - my-awesome-repo-1
        - my-awesome-repo-2
        - my-awesome-repo-3
```

can change location of the file by setting the following env variable: `$SPACES_CONFIG`

path to the directory will be stored in clipboard for easy navigation afterwards

### Purge

`space purge`

same as rm -rf $HOME/spaces
