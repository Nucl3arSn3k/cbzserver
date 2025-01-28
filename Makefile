CC=cargo
FC=npm
FRONTEND_DIR=frontend

.PHONY: dev
dev:
	$(CC) run && \
	cd $(FRONTEND_DIR) && $(FC) run dev