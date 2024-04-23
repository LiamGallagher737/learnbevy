## Metrics Testing

The [docker-compose.yml](./docker-compose.yml) contains a Prometheus service for scaping the metrics endpoint and a Grafana service for visualizing the data. You can run them both with `docker compose up`.

### Prometheus

Once running, Prometheus should start scraping the metrics as defined in [prometheus.yml](./prometheus.yml). You can make sure it's working by going to http://localhost:9090/targets. The target's state should be `up`.

### Grafana

After confirming that Prometheus is functioning you can head over to Grafana at http://localhost:3000 and sign it with the default username and password, "admin".

The first step ia to add the Prometheus instance as a data source. Go to http://localhost:3000/connections/datasources/new and select Prometheus. On the configuration page, under connection, set the url to `http://localhost:9090`. At the bottom of the page click "Save & test" and it should give a successful response.

To add the example dashboard you can go to http://localhost:3000/dashboard/import and paste the contents of [example-dashboard.json](./example-dashboard.json) in to the "Import via dashboard JSON model" text area. After clicking load it will ask which Prometheus data source to use, select the one we added prior, it should be the only option. Import it and everything should be fully set up.
