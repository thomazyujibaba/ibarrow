#!/usr/bin/env python3
"""
Script de teste local para o ibarrow
Carrega credenciais do arquivo .env para testes seguros
"""

import os
import sys
from pathlib import Path

# Adiciona o diretório atual ao path para importar o ibarrow
sys.path.insert(0, str(Path(__file__).parent))

try:
    from dotenv import load_dotenv

    print("✅ python-dotenv encontrado")
except ImportError:
    print("❌ python-dotenv não encontrado. Instalando...")
    os.system("pip install python-dotenv")
    from dotenv import load_dotenv

# Carrega variáveis do arquivo .env
env_file = Path(__file__).parent / ".env"
if not env_file.exists():
    print(f"❌ Arquivo .env não encontrado em {env_file}")
    print("📝 Copie o arquivo env.example para .env e configure suas credenciais:")
    print("   cp env.example .env")
    print("   # Edite o arquivo .env com suas credenciais reais")
    sys.exit(1)

load_dotenv(env_file)

# Obtém credenciais das variáveis de ambiente
DSN = os.getenv("DSN")
DB_USER = os.getenv("DB_USER")
DB_PASSWORD = os.getenv("DB_PASSWORD")
TEST_SQL = os.getenv("TEST_SQL", "SELECT FIRST 5 * FROM RDB$DATABASE")

if not all([DSN, DB_USER, DB_PASSWORD]):
    print("❌ Credenciais incompletas no arquivo .env")
    print("📝 Verifique se DSN, DB_USER e DB_PASSWORD estão configurados")
    sys.exit(1)

print(f"🔗 Conectando ao DSN: {DSN}")
print(f"👤 Usuário: {DB_USER}")
print(f"🔍 SQL de teste: {TEST_SQL}")
print()

try:
    # Tenta importar o ibarrow
    try:
        import ibarrow

        print("✅ ibarrow importado com sucesso")
    except ImportError:
        print("❌ ibarrow não encontrado. Tentando instalar o wheel local...")
        # Procura por arquivos .whl na pasta dist
        dist_path = Path(__file__).parent / "dist"
        if dist_path.exists():
            whl_files = list(dist_path.glob("*.whl"))
            if whl_files:
                latest_whl = max(whl_files, key=os.path.getctime)
                print(f"📦 Instalando {latest_whl.name}...")
                os.system(f"pip install {latest_whl}")
                import ibarrow

                print("✅ ibarrow instalado e importado com sucesso")
            else:
                print("❌ Nenhum arquivo .whl encontrado na pasta dist")
                print("🔧 Execute: maturin build --release --out dist")
                sys.exit(1)
        else:
            print("❌ Pasta dist não encontrada")
            print("🔧 Execute: maturin build --release --out dist")
            sys.exit(1)

    # Configuração padrão para testes
    config = ibarrow.QueryConfig(
        batch_size=1000,
        max_text_size=65536,
        max_binary_size=65536,
        read_only=True,
        connection_timeout=30,
        query_timeout=60,
        isolation_level="read_committed",
    )

    print("🧪 Testando diferentes métodos...\n")

    # Teste 1: Arrow IPC
    print("1️⃣ Testando query_arrow_ipc...")
    try:
        result_bytes = ibarrow.query_arrow_ipc(
            DSN, DB_USER, DB_PASSWORD, TEST_SQL, config
        )
        print(f"   ✅ Sucesso! Retornou {len(result_bytes)} bytes")
    except Exception as e:
        print(f"   ❌ Erro: {e}")

    # Teste 2: Polars DataFrame
    print("\n2️⃣ Testando query_polars...")
    try:
        df = ibarrow.query_polars(DSN, DB_USER, DB_PASSWORD, TEST_SQL, config)
        print(f"   ✅ Sucesso! DataFrame shape: {df.shape}")
        print(f"   📊 Colunas: {df.columns}")
        print("   🔍 Primeiras linhas:")
        print(df.head(3))
    except Exception as e:
        print(f"   ❌ Erro: {e}")

    # Teste 3: Pandas DataFrame
    print("\n3️⃣ Testando query_pandas...")
    try:
        df = ibarrow.query_pandas(DSN, DB_USER, DB_PASSWORD, TEST_SQL, config)
        print(f"   ✅ Sucesso! DataFrame shape: {df.shape}")
        print(f"   📊 Colunas: {list(df.columns)}")
        print("   🔍 Primeiras linhas:")
        print(df.head(3))
    except Exception as e:
        print(f"   ❌ Erro: {e}")

    # Teste 4: Arrow C Data Interface
    print("\n4️⃣ Testando query_arrow_c_data...")
    try:
        # Teste com retorno de PyCapsules
        capsules = ibarrow.query_arrow_c_data(
            DSN, DB_USER, DB_PASSWORD, TEST_SQL, config, return_dataframe=False
        )
        print(f"   ✅ Sucesso! Retornou PyCapsules: {type(capsules)}")

        # Teste com retorno de DataFrame
        df = ibarrow.query_arrow_c_data(
            DSN, DB_USER, DB_PASSWORD, TEST_SQL, config, return_dataframe=True
        )
        print(f"   ✅ Sucesso! DataFrame shape: {df.shape}")
    except Exception as e:
        print(f"   ❌ Erro: {e}")

    print("\n🎉 Testes concluídos!")

except Exception as e:
    print(f"💥 Erro geral: {e}")
    import traceback

    traceback.print_exc()
