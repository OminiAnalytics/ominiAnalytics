# Omini

## User analytics

### Requirements

-   Cargo and Rust

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

-   Postgres
    Omini use [Postgres](https://www.postgresql.org/) as a database.
    ```bash
    sudo apt-get install postgresql postgresql-contrib
    ```
-   Diesel cli
    Omini use [Diesel](https://diesel.rs/) for migrations
    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```
    Troubleshooting :
    Look at [this link](https://github.com/diesel-rs/diesel/issues/2026#issuecomment-479716695) and [this one](https://github.com/diesel-rs/diesel/issues/2026#issuecomment-1126615465)

### Init Database

This example assumes you are using `your_password` as the password for the database.

1.  Create user `omini_user`

    ```bash
    sudo -u postgres createuser -P omini_user
    sudo adduser omini_user
    ```

2.  Create database `omini_db`

    ```sql
    sudo -u postgres createdb -O omini_user omini_db
    ```

3.  Update /etc/postgresql/`version`/main/pg_hba.conf

    Add the following line to pg_hba.conf:

    ```
    local   omini_db        omini_user                              scram-sha-256
    ```

4.  Update `.env`

    Add the following line to `.env` :

    ```
    DATABASE_URL=postgres://omini_user:your_password@localhost/omini_db
    SERVER_HOST=0.0.0.0:5000
    ```

5.  Grand necessary permissions for the user `omini_user`

    ```sql
    GRANT ALL PRIVILEGES ON DATABASE omini_db TO omini_user;
    ```

6.  Init database

    ```bash
    diesel migration run
    ```
