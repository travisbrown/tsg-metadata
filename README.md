# Twitter Stream Grab metadata

[![Rust build status](https://img.shields.io/github/actions/workflow/status/travisbrown/tsg-metadata/ci.yaml?branch=main)](https://github.com/travisbrown/tsg-metadata/actions)
[![Coverage status](https://img.shields.io/codecov/c/github/travisbrown/tsg-metadata/main.svg)](https://codecov.io/github/travisbrown/tsg-metadata)

This repository contains metadata associated with the [Twitter Stream Grab][tsg],
which is a collection of billions of tweets archived over a decade by [the Archive Team][at],
"a rogue archivist collective dedicated to saving copies of rapidly dying or deleted websites for the sake of history and digital heritage".
The Twitter Stream Grab is published by the [Internet Archive][ia].

Please note that the author of this repository is not affiliated with the Internet Archive or the Archive Team.
This repository does not contain any code that accesses any Twitter API or other Twitter services.
It also does not contain any data from Twitter. It only provides tools for working with files published by the Internet Archive.

The `sources` directory contains metadata files for the Twitter Stream Grab copied from the Internet Archive.
Please note that otherwise the code provided in this repository is **not** "open source",
but the source is available for use and modification by individuals, non-profit organizations, and worker-owned cooperatives
(see the [license section](#license) below for details).

## License

This software is published under the [Anti-Capitalist Software License][acsl] (v. 1.4).

[acsl]: https://anticapitalist.software/
[at]: https://archive.org/details/archiveteam
[ia]: https://archive.org/
[tsg]: https://archive.org/details/twitterstream
