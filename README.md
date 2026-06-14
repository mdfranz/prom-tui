# prom-tui

> [!NOTE]
> This project modernizes and fixes an older codebase that hasn't been updated in years, updating deprecated dependencies and adding features like Prometheus Summary metric support.

A simple terminal ui application to visualize Prometheus metrics.

## Usage

Start with 'cargo run' and quit by pressing 'q'.

You can provide the endpoint to scrape in 2 ways:
  1. as CLI argument
  2. as env variable

In the case of the CLI argument run:

```bash
cargo run -- --endpoint "http://localhost:8081/metrics"
```

with the env variable
```bash
PROMTUI_ENDPOINT=http://localhost:8081/metrics cargo run
```

If no endpoint is provided the default value is http://localhost:8080/metrics

## Local development

To test the application locally, you can run a Prometheus-compatible metrics generator like [Avalanche](https://github.com/prometheus-community/avalanche).

Using Docker:
```bash
docker run -p 8080:8080 quay.io/prometheuscommunity/avalanche:main --port=8080
```

Then run `prom-tui`:
```bash
cargo run
```

