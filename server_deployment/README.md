# Application deployment procedures

## Bastion host
A [startup script](bastion_install.sh) is provided for use in configuring a bastion host. This can be used as the 
[User Data](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html?icmpid=docs_ec2_console) 
for an EC2 instance.

## Database

Database migrations are found in [the `db` directory](../db) and managed with [Flyway](https://flywaydb.org/).
To update a migration:
```shell
flyway \
  -url="jdbc:postgresql://{HOST}:5432/{DATABASE}?user={USER}" \
  -locations="filesystem:./db" \
  migrate
```
The user's password will be queried before running.
To check the status of migrations without making any changes use the command `info` instead.
Naming of migration scripts should follow the format:

`V{migration number}__{descriptive name}.sql`

Example: `V1__create_wager_table.sql`

For debugging within the database prefer psql.
```shell
psql -h {HOST} -U {USER} -W"
```

## Serverless
The application is expected to be deployed as a Lambda instance with network security configured for the database.

Expected environment variables:
- DB_CONNECTION_STRING - connection string formatted for use by [SQLx Postgres driver](https://github.com/launchbadge/sqlx)
- DISCORD_PUBLIC_KEY - the public key used for Ed25519 verification of inbound calls