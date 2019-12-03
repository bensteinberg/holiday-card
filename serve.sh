#!/bin/bash

# run backend
export DEV_SERVER=1
systemfd --no-pid -s http::8001 -- cargo watch -i frontend -x run &

# run frontend
cd frontend && npm run serve

# cleanup on exit
KillJobs() {
    for job in $(jobs -p); do
        kill -s SIGTERM $job > /dev/null 2>&1 || (sleep 10 && kill -9 $job > /dev/null 2>&1 &)
    done
}
trap KillJobs SIGINT SIGTERM EXIT