# LexOffice Analytics

This project is used to fetch data from a Lexoffice account using the [Lexoffice Public API](https://developers.lexoffice.io/docs/#lexoffice-api-documentation).\
The fetched data is stored inside a Postgres Database and can then be used to perform data analytics based on this data.

## Project Structure

- [lexoffice-cli](/lexoffice-sync/): A CLI application in Rust, which can be used to fetch the latest data from Lexoffice and save it to the database
- [db](/db/): PostgreSQL data (only used if hosting locally)

## Open Tasks

- [ ] Sync lexoffice data with postgres database
- [ ] Create a Dashboard application to visualize stored data
