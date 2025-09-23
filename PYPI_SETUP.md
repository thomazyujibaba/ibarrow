# 📦 Guia de Publicação no PyPI

## 🔐 Configuração Inicial

### 1. Criar conta no PyPI
- Acesse: https://pypi.org/account/register/
- Crie uma conta (pode usar a mesma do GitHub)
- **Ative 2FA** (obrigatório para uploads)

### 2. Gerar Token de API
1. Acesse: https://pypi.org/manage/account/
2. Vá em "API tokens"
3. Clique em "Add API token"
4. **Nome**: `ibarrow-upload`
5. **Scope**: `Entire account (all projects)`
6. **Copie o token** (formato: `pypi-...`)

### 3. Configurar GitHub Secrets
No seu repositório GitHub:
1. Vá em **Settings** → **Secrets and variables** → **Actions**
2. Clique em **New repository secret**
3. **Name**: `PYPI_API_TOKEN`
4. **Value**: Cole o token do PyPI (pypi-...)

## 🚀 Publicação Automática via GitHub Actions

O projeto já está configurado para publicação automática! Basta:

### 1. Fazer um commit com tag de versão
```bash
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
git push origin main --tags
```

### 2. O GitHub Actions fará automaticamente:
- ✅ Compilar para Windows, Linux, macOS
- ✅ Gerar wheels para Python 3.8-3.12
- ✅ Publicar no PyPI
- ✅ Criar release no GitHub

## 🧪 Teste Local (Opcional)

Se quiser testar localmente antes:

### 1. Instalar twine
```bash
pip install twine
```

### 2. Testar upload (sem publicar)
```bash
twine check dist/*
twine upload --repository testpypi dist/*
```

### 3. Publicar no PyPI real
```bash
twine upload dist/*
```

## 📋 Checklist de Publicação

- [ ] ✅ Código compilando sem erros
- [ ] ✅ Wheel gerado em `dist/`
- [ ] ✅ `pyproject.toml` configurado
- [ ] ✅ README.md atualizado
- [ ] ✅ Conta PyPI criada
- [ ] ✅ Token API gerado
- [ ] ✅ GitHub Secret configurado
- [ ] ✅ Tag de versão criada

## 🎯 Após a Publicação

Qualquer pessoa poderá instalar com:
```bash
pip install ibarrow
```

E usar:
```python
import ibarrow
df = ibarrow.query_polars("DSN", "user", "pass", "SELECT * FROM table")
```

## 🔄 Atualizações Futuras

Para novas versões:
1. Atualize a versão no `pyproject.toml`
2. Faça commit e tag: `git tag v0.2.0`
3. Push: `git push origin main --tags`
4. GitHub Actions publica automaticamente!
