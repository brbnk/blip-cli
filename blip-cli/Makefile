APP_NAME=blip
TARGET_DIR=target/release
BIN_PATH=$(TARGET_DIR)/$(APP_NAME)
RENAMED_BIN=$(TARGET_DIR)/$(APP_NAME)
INSTALL_PATH=/usr/bin/$(APP_NAME)

.PHONY: all build install rust

all: rust build install

rust:
	@echo "🔧 Verificando se o Rust está instalado..."
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "🔧 Rust não encontrado. Instalando..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		source $$HOME/.cargo/env; \
	else \
		echo "✅ Rust já está instalado."; \
	fi

build:
	@echo "🚀 Compilando o projeto..."
	cargo build --release

install: 
	@echo "📦 Instalando em $(INSTALL_PATH)..."
	sudo cp $(RENAMED_BIN) $(INSTALL_PATH)
	@echo "✅ Instalação concluída: $(INSTALL_PATH)"
