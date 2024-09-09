#!/bin/bash
flyway \
  -url="jdbc:postgresql://localhost:5432/pog_server?user=pog_user" \
  -locations="filesystem:./db/migrations/" \
  migrate