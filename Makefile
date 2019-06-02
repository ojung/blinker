.PHONY: all build deploy

all: build deploy

build:
	docker run --rm --mount=type=bind,source="$(shell pwd)"/artifacts,target=/artifacts/ blinker:latest sh -c "cargo build --release; cp target/release/blinker /artifacts/"

deploy:
	ssh pi@192.168.2.104 tmux kill-session -t "blinker" || true
	scp artifacts/blinker pi@192.168.2.104:bin/
	ssh pi@192.168.2.104 tmux kill-session -t "blinker" || true
	ssh pi@192.168.2.104 tmux new -d -s "blinker" "sudo bin/blinker"
