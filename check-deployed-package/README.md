# LogDash Rust SDK Demo

Use command below to test if the published SDK works:

```
LOGDASH_API_KEY=<your-api-key> LOGDASH_PERSONAL_API_KEY=<your-personal-api-key> ./check-deployed-package/run.sh
```

`LOGDASH_API_KEY` sends the logs and metrics.
`LOGDASH_PERSONAL_API_KEY` reads them back: a personal API key with `logs:read` and `metrics:read`, restricted to the project of `LOGDASH_API_KEY`.
Set `LOGDASH_VERSION` to check a version other than the newest.
