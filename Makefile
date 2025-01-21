CC=cargo
FC = npm
FRONTEND_DIR = frontend
.PHONY: dev
dev:
	@(cd $(FRONTEND_DIR) && $(FC) run dev) & \
	$(CC) run & \
	wait
