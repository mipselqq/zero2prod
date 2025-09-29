Docker containers are so self-contained that they have their own networks. You can intrude them from the host machine by publishing (exposing) ports.

To access a container from another container, you need to specify
its address in the default docker network (bridge), but it's highly
recommended to use Docker's DNS that satisfies persistency, whilst
IP's are unstable.

Even better, it's possible to use docker-compose to easily merge
network of the two containers.
