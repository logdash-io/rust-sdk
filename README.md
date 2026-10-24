# logdash - RUST SDK

Logdash is a zero-config observability platform. This package serves an rust interface to use it.

## Pre-requisites

Setup your free project in less than 2 minutes at [logdash.io](https://logdash.io/)

## Installation

```
cargo add logdash
```

## Logging

get API KEY from env LOGDASH_API_KEY

```rust
let (logger, _) = logdash::create_logdash(logdash::Config::default());

// Send an info log message
logger.info("Rust SDK example");
```

get API KEY from config

```rust
let (l, _) = logdash::create_logdash(logdash::Config::default().api_key("Your Api Key".into()));

// Send an info log message
l.info("Rust SDK example with API key");
```

## Metrics

```rust
let (_, metrics) = logdash::create_logdash(logdash::Config::default());

// create a metric
metrics.set("user".into(), 0.0);

// or increment / decrement by
metrics.mutate("user".into(), 1.0);
```

get API KEY from config

```rust
let (_, m) = logdash::create_logdash(logdash::Config::default().api_key("Your Api Key".into()));

// create a metric
m.set("user".into(), 0.0);

// or increment / decrement by
m.mutate("user".into(), 1.0);
```

## Events

```rust
logdash::create_logdash(logdash::Config::default());

// track that a signed-in user ran an agent
logdash::track(
    "agent_run",
    &[("agent", "support".into()), ("steps", 4.into()), ("success", true.into())],
    Some(&user.id),
);
```

Events show up in the domain's Analytics as goals, with a daily trend.
Pass the user ID for signed-in users, so one person counts once across the website and apps.
Without it, Logdash counts a visitor of that app that changes every day.
Never put personal data in props.

## Flush

Logs, metrics and events are sent in the background.
Call `flush` before your app exits, so nothing queued is lost.

```rust
// wait until the logs, metrics and events queued so far are sent
logdash::flush();
```

Each request fails after 5 seconds without an answer, so `flush` returns even when the API is unreachable.

## Apps

A domain key is shared by all apps of a domain, for example a desktop app, an API and a landing page.
Set `app` to name this one, so the dashboard can filter logs and metrics by app.

```rust
let (logger, metrics) = logdash::create_logdash(
    logdash::Config::default()
        .api_key("Your Domain Key".into())
        .app("desktop".into()),
);

// both are sent with "app": "desktop"
logger.info("Desktop app started");
metrics.set("users".into(), 1.0);
```

Older service keys ignore `app`.

## View

To see the logs or metrics, go to your project dashboard

![logs](docs/logs.png)
![delta](docs/delta.png)

## Configuration

| Parameter | Required | Default | Description                                                                                                                         |
| --------- | -------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `api_key` | no       | -       | Api key used to authorize against logdash servers. If you don't provide one, logs and events will be logged into local console only |
| `api_url` | no       | -       | Custom API host, useful with self-hosted instances                                                                                  |
| `verbose` | no       | -       | Useful for debugging purposes                                                                                                       |
| `app`     | no       | -       | Name of the app sending logs, metrics and events, for example `desktop`. See [Apps](#apps)                                          |

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Support

If you encounter any issues, please open an issue on GitHub or let us know at [contact@logdash.io](mailto:contact@logdash.io).
