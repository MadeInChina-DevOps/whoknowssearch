# monitoring/Dockerfile
FROM prom/prometheus as prometheus
COPY prometheus/prometheus.yml /etc/prometheus/prometheus.yml

FROM grafana/grafana as grafana
COPY grafana/datasource.yml /etc/grafana/provisioning/datasources/datasource.yml