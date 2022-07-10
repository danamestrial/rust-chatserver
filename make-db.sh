#!/bin/bash
if [ $# -lt 1 ]
then
	echo "Usage: ./make-db.sh <password>"
	exit
fi

docker run --name postgresdb -p 5432:5432 -e POSTGRES_PASSWORD=superpassword1234 --restart=always -d postgres
