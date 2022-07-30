# reverse-osmosis

`reverse-osmosis` (`ro`) filters sections of files in various ways.
man that name is clunky. i'll workshop it.

here are some things you can do with the sections:

- sort
- align by delimiter
- filter with executable
    - e.g. format with jq
    - useful for formatting snippets of other languages embedded in a host
      language source file
- "verify" with executable
    - e.g. verify snippets compile with no warnings
    - e.g. verify passes some linter
- verify lines match regex


## syntax

```
// reverse-osmosis:sort{unique: true}
a
b
c
// reverse-osmosis:end

<!-- reverse-osmosis:filter{executable: nixfmt} -->
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    # ...
}
<!-- reverse-osmosis:end -->
```

- Punctuation `reverse-osmosis:` Filter [ FilterArgs ] [ Punctuation ]
- maybe we want a "do the rest of the file" syntax too
- it'd be nice to have a syntax for "arguments" to the filters, too. inline
  yaml??? lol
- multiline args would be nice too but would significantly increase complexity
