# auftritt

### Preparing

`auftritt` uses the MaxMind GeoLite2 free country database. More details can be found on their website [here](https://dev.maxmind.com/geoip/geolite2-free-geolocation-data). Download the database and extract it.

### Running

```bash
# lets assume the database is in the data directory `./data/GeoLite2-Country.mmdb`
cargo run --release ./data/GeoLite2-Country.mmdb
# lookup ip
curl http://localhost:8080/1.1.1.1
# lookup yourself (localhost results in bad request)
curl http://localhost:8080/me
```

#### Response

```json
{
  "details": {
    "continent": {
      "code": "OC",
      "geoname_id": 6255151,
      "names": {
        "de": "Ozeanien",
        "en": "Oceania",
        "es": "Oceanía",
        "fr": "Océanie",
        "ja": "オセアニア",
        "pt-BR": "Oceania",
        "ru": "Океания",
        "zh-CN": "大洋洲"
      }
    },
    "country": {
      "geoname_id": 2077456,
      "is_in_european_union": null,
      "iso_code": "AU",
      "names": {
        "de": "Australien",
        "en": "Australia",
        "es": "Australia",
        "fr": "Australie",
        "ja": "オーストラリア",
        "pt-BR": "Austrália",
        "ru": "Австралия",
        "zh-CN": "澳大利亚"
      }
    },
    "registered_country": {
      "geoname_id": 2077456,
      "is_in_european_union": null,
      "iso_code": "AU",
      "names": {
        "de": "Australien",
        "en": "Australia",
        "es": "Australia",
        "fr": "Australie",
        "ja": "オーストラリア",
        "pt-BR": "Austrália",
        "ru": "Австралия",
        "zh-CN": "澳大利亚"
      }
    },
    "represented_country": null,
    "traits": null
  },
  "ip": "1.1.1.1"
}
```

### Routes

##### `/me`

| Status code | Problems |
|-|-|
| `200` | none |
| `400` | your ip couldn't be extracted correctly |
| `404` | your ip hasn't been found in the database |
| `500` | we had internal issues looking into the database |

##### `/:ip`

| Status code | Problems |
|-|-|
| `200` | none |
| `400` | the given ip isn't a valid ip |
| `404` | the requested ip hasn't been found in the database |
| `500` | we had internal issues looking into the database |

### Benchmark

- The api defaults to the number of cpu cores as workers.
- Tested Hardware: `AMD Ryzen 9 5900x 12cores 24threads 2.2GHz-3.7GHz`.


#### Full Benchmark

12threads used for the api, 12 threads used for the `wrk` benchmark.

```
➜  ~ wrk -t 12 -c 1000 --latency -d 10s http://127.0.0.1:8080/1.1.1.1
Running 10s test @ http://127.0.0.1:8080/1.1.1.1
  12 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.63ms    1.44ms  83.71ms   98.92%
    Req/Sec    52.45k     1.44k   56.87k    84.83%
  Latency Distribution
     50%    1.53ms
     75%    1.78ms
     90%    2.05ms
     99%    3.14ms
  6262581 requests in 10.02s, 5.26GB read
Requests/sec: 624954.13
Transfer/sec:    537.59MB
```

#### Minimal Benchmark

12threads used for the api, 1 thread used for the `wrk` benchmark.

```
➜  ~ wrk -t 1 -c 1 --latency -d 10s http://127.0.0.1:8080/1.1.1.1
Running 10s test @ http://127.0.0.1:8080/1.1.1.1
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    29.80us    4.80us 535.00us   82.25%
    Req/Sec    32.76k     2.24k   43.78k    83.17%
  Latency Distribution
     50%   29.00us
     75%   32.00us
     90%   35.00us
     99%   41.00us
  329355 requests in 10.10s, 283.32MB read
Requests/sec:  32611.43
Transfer/sec:     28.05MB
```

### License

I don't really care. Do whatever you like to do.


##### MaxMind license stuff

This product includes GeoLite2 data created by MaxMind, available from
<a href="https://www.maxmind.com">https://www.maxmind.com</a>.