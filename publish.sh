#! /bin/bash
set -e

docker build -t rbn_export .
docker tag rbn_export ewpratten/rbn_exporter:latest
docker push ewpratten/rbn_exporter