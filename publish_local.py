#!/usr/bin/env python3
"""
Script para publicação local no PyPI (opcional)
Use apenas se quiser testar antes da publicação automática
"""

import os
import sys
import subprocess
from pathlib import Path


def run_command(cmd, description):
    """Executa comando e mostra resultado"""
    print(f"🔧 {description}...")
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"❌ Erro: {result.stderr}")
        return False
    print(f"✅ {description} concluído")
    return True


def main():
    print("🚀 Publicação Local do ibarrow no PyPI")
    print("=" * 50)

    # Verificar se estamos no diretório correto
    if not Path("pyproject.toml").exists():
        print("❌ Execute este script no diretório raiz do projeto")
        sys.exit(1)

    # 1. Limpar builds anteriores
    print("\n1️⃣ Limpando builds anteriores...")
    if Path("dist").exists():
        import shutil

        shutil.rmtree("dist")
    if Path("target").exists():
        import shutil

        shutil.rmtree("target")

    # 2. Compilar o projeto
    if not run_command("cargo build --release", "Compilação Rust"):
        sys.exit(1)

    # 3. Gerar wheel
    if not run_command("maturin build --release --out dist", "Geração do wheel"):
        sys.exit(1)

    # 4. Verificar se wheel foi gerado
    dist_files = list(Path("dist").glob("*.whl"))
    if not dist_files:
        print("❌ Nenhum wheel encontrado na pasta dist")
        sys.exit(1)

    print(f"📦 Wheel gerado: {dist_files[0].name}")

    # 5. Verificar token PyPI
    token = os.getenv("PYPI_API_TOKEN")
    if not token:
        print("❌ Token PyPI não encontrado")
        print("📝 Configure a variável de ambiente PYPI_API_TOKEN")
        print("   export PYPI_API_TOKEN=pypi-...")
        sys.exit(1)

    # 6. Verificar wheel
    if not run_command("pip install twine", "Instalação do twine"):
        sys.exit(1)

    if not run_command("twine check dist/*", "Verificação do wheel"):
        sys.exit(1)

    # 7. Perguntar se quer publicar
    print("\n⚠️  ATENÇÃO: Isso publicará no PyPI real!")
    print("📦 Arquivo:", dist_files[0].name)
    print("🔗 PyPI: https://pypi.org/project/ibarrow/")

    response = input("\n🤔 Deseja continuar? (y/N): ").lower().strip()
    if response != "y":
        print("❌ Publicação cancelada")
        sys.exit(0)

    # 8. Publicar
    if not run_command("twine upload dist/*", "Publicação no PyPI"):
        sys.exit(1)

    print("\n🎉 Publicação concluída com sucesso!")
    print("📦 Pacote disponível em: https://pypi.org/project/ibarrow/")
    print("💻 Instalar com: pip install ibarrow")


if __name__ == "__main__":
    main()
