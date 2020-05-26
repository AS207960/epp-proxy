#!/usr/bin/env bash

VERSION=$(sentry-cli releases propose-version || exit)

kubectl apply -f cert.yaml || exit
sed -e "s/(version)/$VERSION/g" < deploy.yaml | kubectl apply -f - || exit

sentry-cli releases --org as207960 deploys $VERSION new -e prod
