[![Ci](https://github.com/Xannden/mu-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Xannden/mu-rs/actions/workflows/rust.yml)

A basic library to scrape [MangaUpdates](http://www.mangaupdates.com) pages

>Right now only some of MangaUpdates pages have been implemented

## Usage

In the simplest form you can get series info like this

```no_run
let client = MuClient::new();
let series = client.series(SeriesId(15));
```

See [SeriesId] for information on getting the series id from a page on mangaupdates
