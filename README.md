# spinning

My serverless WebAssembly explorations using [Fermyon Spin](https://www.fermyon.com/spin).

## redirecta

Redirects all HTTP(S) requests to a specific URL while preserving path (but not query – I didn't bother supporting that).

For example, when `redirecta` is hosted at [hypermedia.dev](https://hypermedia.dev), then a request to
 [hypermedia.**dev**/book/contents](https://hypermedia.dev/book/contents) gets redirected to
[hypermedia.**systems**/book/contents](https://hypermedia.systems/book/contents).

## traccumulator

Accumulates current tracks from different radio station APIs and would ideally create playlists in Spotify for me to
listen.

Right now only returns current track (sometimes also recent tracks) that's playing at a specific radio station.

Later, when [Fermyon Cloud](https://www.fermyon.com/cloud) supports secrets, I intend to add database and component(s)
for Spotify integration too.

Ideally, the trigger would be cron instead of HTTP – I intend to explore that later, too.

### Radio stations

| Station                                         | Component path                                               | HTTP path         |
|-------------------------------------------------|--------------------------------------------------------------|-------------------|
| [Raadio Tallinn](https://raadiotallinn.err.ee/) | [traccumulator/raadio_tallinn](traccumulator/raadio_tallinn) | `/raadio-tallinn` |
| [Groove FM](https://www.supla.fi/groovefm)      | [traccumulator/supla_fi](traccumulator/supla_fi)             | `/groove-fm`      |
| [Loop](https://www.supla.fi/loop)               | [traccumulator/supla_fi](traccumulator/supla_fi)             | `/loop`           |
