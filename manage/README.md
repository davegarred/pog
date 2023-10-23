# Application deployment procedures

## Bastion host
All resources should be secured within a private subnet with a bastion host for debug access available within a 
connected public subnet. A [startup script](bastion_install.sh) is provided for use as the 
[User Data](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html?icmpid=docs_ec2_console) for an 
EC2 instance.

## Database

Database migrations are managed with [Flyway](https://flywaydb.org/).
To update a migration:
```shell
flyway \
  -url="jdbc:postgresql://{HOST}:5432/{DATABASE}?user={USER}" \
  -locations="filesystem:." \
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
- DB_CONNECTION_STRING - connection string formatted for use by [Golang Postgres driver](https://pkg.go.dev/github.com/lib/pq#hdr-Connection_String_Parameters)
- DISCORD_PUBLIC_KEY - the public key used for Ed25519 verification of inbound calls