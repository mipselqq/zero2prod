Docker containers are so self-contained that they have their own networks. You can intrude them from the host machine by publishing (exposing) ports.

To access a container from another container, you need to specify
its address in the default docker network (bridge), but it's highly
recommended to use Docker's DNS that satisfies persistency, whilst
IP's are unstable. Note that bridge doesn't support DNS for historical reasons.

Even better, it's possible to use docker-compose to easily merge
network of the two containers.

tag = image name

Musl cross-compilation for alpine doesn't seem to worth it, bc it increases compile times and introduces serious inconsistency between dev and prod runtimes.

Both ENTRYPOINT and CMD can be used to define a `command` to run when running a container, but the former can be extended, and the latter can be overrided when specifying
docker run command arguments.
