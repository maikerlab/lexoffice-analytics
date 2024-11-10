# lexoffice Analytics

Save and visualize data from the [lexoffice Public API](https://developers.lexoffice.io/docs/#lexoffice-api-documentation).

![](docs/sys-arch.svg)

This project contains a Rust workspace with the following components:

- [lexoffice-cli](/lexoffice-cli): User application which supports commands to fetch and save data from lexoffice. Data will be saved to a MongoDB Cluster, self-hosting with PostgreSQL may be supported in the future.
- [lexoffice-models](/lexoffice-models): Contains types for stored data and functions for converting to/from lexoffice types
- [openapi](/openapi): Library containing types from lexoffice API and functions for executing API calls. Generated by [OpenAPI Generator](https://openapi-generator.tech/) using the OpenAPI specification in [lexoffice-api.yml](lexoffice-api.yml)
- *Web App*: May be implemented in the future to support visualizing or interacting with the stored data


## lexoffice-cli

When running the CLI, the following environment variables are expected to be set:

- `DATABASE_URL`: Connection string to your MongoDB instance (`mongodb://[user]:[password]@[host]:[port]/[connection options]`)
- `LEXOFFICE_APIKEY`: API Key for Lexoffice account (create one [here](https://app.lexoffice.de/addons/public-api))

### Usage

The following subcommands are supported at the moment:

- `sync`: Fetch vouchers from lexoffice and save them into MongoDB
- `help`: Show help for usage of the CLI

```shell
$ cargo run -- sync <VOUCHER_TYPE> --from <FROM_DATE> --to <TO_DATE>
```

The argument `VOUCHER_TYPE` is optional and currently supports:

- `all`
- `invoices`

If `VOUCHER_TYPE` is not provided, all voucher types will be synced.

A specific date range can be provided with the optional arguments `--from` and `--to` in the format `YYYY-MM-DD`.
If only one is provided, the maximum start/end date will be used for the other argument.

## openapi

Re-generating the openapi Library Crate is only necessary after modifiying the OpenAPI spec ([lexoffice-api.yml](lexoffice-api.yml)).

To do this, first install the required dependencies for Node.js (only once).
Then the OpenAPI client can be generated using the `generate` script:

```shell
$ npm install
$ npm run generate
```

## Run in Docker

For the current state of the CLI application, it's kind of pointless to run it in Docker, but the idea is that it could be extended, e.g. to expose an API to a Web App, from where synchronizing data could be triggered and the stored data could be visualized or even be edited/extended.
However, running in Docker gives you the benefit to self-host your MongoDB instance.

For the `db` Service, you need to set the environment variables `MONGO_INITDB_ROOT_USERNAME` and `MONGO_INITDB_ROOT_PASSWORD` and then modify your MongoDB connection string (`DATABASE_URL`), containing the correct authentication credentials.

Then simply run `docker-compose up db` and interact with the MongoDB container by running commands from the `cli` service, e.g.:
`docker-compose run cli sync invoices`.
