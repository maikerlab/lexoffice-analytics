# LexOffice CLI

This service implements a CLI application, which can be used to interact with the [Lexoffice Public API](https://developers.lexoffice.io/docs/).

Commands:

- `lexoffice-cli sync`: Fetches data from the API and saves it to the database

## Interacting with the Lexoffice API

*A client can make up to 2 requests per second to the lexoffice API.* ([source](https://developers.lexoffice.io/docs/#api-rate-limits))

- [ ] Solution: **TODO**

## OpenAPI

[lexoffice-api.yml](lexoffice-api.yml) contains an OpenAPI specification for the Lexoffice Public API.\
Using [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator), a Rust client is generated.\
This service (**lexoffice-sync**) uses this library to get data from Lexoffice and save it into the database.

We can (re)create the client using this command:

```shell
openapi-generator-cli generate -i lexoffice-api.yml -g rust -o openapi
```

## Database

We can create the database and run mirations using Diesel CLI.\
Requirement: `DATABASE_URL` must be set as environment variable (e.g.: `postgres://bunu:bunu@localhost:5434/bunu`).
More info at [diesel.rs](https://diesel.rs/guides/getting-started).

Create database and run all pending migrations:

```shell
diesel setup
diesel migration run
```

### Tables

- vouchers (from `api.lexoffice.io/v1/voucherlist` endpoint)
- invoices (from `api.lexoffice.io/v1/invoices/{id}` endpoint)
- addresses
- line_items
- products
