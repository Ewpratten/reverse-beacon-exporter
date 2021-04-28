# Prometheus exported for the Reverse Beacon Network

This tool exports data from the [Reverse Beacon Network](http://reversebeacon.net/) tcp stream as something [Prometheus](https://prometheus.io/) can handle. The main goal is to track CQ calls over time in a [Grafana](https://grafana.com/) dashboard.

This tool can be ran from `cargo` or through the `ewpratten/rbn_exporter:latest` docker image. No configuration required. Data is exposed on port `9814`.