#!/usr/bin/env bash
export CI_COMMIT_BRANCH=WEB-1
export LIVE_DATABASE=app_bambushain_pandas
kubectl kustomize > manifest.yaml
sed -i "s/\$BRANCH/${CI_COMMIT_BRANCH,,}/g" manifest.yaml
kubectl create namespace bamboo-review-${CI_COMMIT_BRANCH,,}
kubectl apply -f manifest.yaml
sleep 60
kubectl wait --for=condition=Ready -n bamboo-review-${CI_COMMIT_BRANCH,,} pods -l bambushain.app/app=postgres --timeout=90s
sed -i "s/\$POSTGRES_PANDAS/${LIVE_DATABASE}/g" sync-database.yaml
sed -i "s/\$BRANCH/${CI_COMMIT_BRANCH,,}/g" sync-database.yaml
kubectl apply -f sync-database.yaml
sleep 60
kubectl wait --for=condition=complete job/sync-database -n bamboo-review-${CI_COMMIT_BRANCH,,} --timeout=90s
sed -i "s/\$BRANCH/${CI_COMMIT_BRANCH,,}/g" migrate.yaml
kubectl apply -f migrate.yaml