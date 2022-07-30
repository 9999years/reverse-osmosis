# reverse-osmosis

`reverse-osmosis` (`ro`) filters sections of files in various ways.
man that name is clunky. i'll workshop it.

here are some things you can do with the sections:

- sort
- align by delimiter
- filter with executable
    - e.g. format with jq
- "verify" with executable
    - e.g. verify snippets compile with no warnings
    - e.g. verify passes some linter
- verify lines match regex


## syntax

- punctuation `reverse-osmosis:sort` [ punctuation ]
- maybe we want a "do the rest of the file" syntax too
- it'd be nice to have a syntax for "arguments" to the filters, too. inline
  yaml??? lol
