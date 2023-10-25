#!/bin/bash

apt update
apt upgrade -y
apt install postgresql-client-14 -y
snap install flyway

reboot now
